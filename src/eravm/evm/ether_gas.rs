//!
//! Translates the value and balance operations.
//!

use crate::context::IContext;
use crate::eravm::context::Context;

///
/// Translates the `gas` instruction.
///
pub fn gas<'ctx>(
    context: &mut Context<'ctx>,
) -> anyhow::Result<inkwell::values::BasicValueEnum<'ctx>> {
    Ok(context
        .build_call(context.intrinsics().gas_left, &[], "gas_left")?
        .expect("Always exists"))
}

///
/// Translates the `value` instruction.
///
pub fn value<'ctx>(
    context: &mut Context<'ctx>,
) -> anyhow::Result<inkwell::values::BasicValueEnum<'ctx>> {
    Ok(context
        .build_call(context.intrinsics().get_u128, &[], "get_u128_value")?
        .expect("Always exists"))
}

///
/// Translates the `balance` instructions.
///
pub fn balance<'ctx>(
    context: &mut Context<'ctx>,
    address: inkwell::values::IntValue<'ctx>,
) -> anyhow::Result<inkwell::values::BasicValueEnum<'ctx>> {
    crate::eravm::evm::call::request(
        context,
        context.field_const(zkevm_opcode_defs::ADDRESS_ETH_TOKEN.into()),
        "balanceOf(uint256)",
        vec![address],
    )
}
