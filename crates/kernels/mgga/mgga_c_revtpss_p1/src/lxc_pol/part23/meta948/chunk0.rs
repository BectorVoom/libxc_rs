//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3133/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3133<F: Float>(t24252: F, t300: F, t1198: F, t1765: F, t68609: F, t16784: F, t6552: F, t20384: F, t5192: F, t24498: F, t3531: F, t20400: F, t5202: F) -> (F, F, F, F, F, F) {
    let t82389 = t300 * t24252;
    let t82391 = F::cast_from(0.5848223622634646207e0_f64) * t82389 * t1198;
    let t82394 = F::cast_from(0.17544670867903938621e1_f64) * t68609 * t1765;
    let t82396 = F::cast_from(0.17544670867903938621e1_f64) * t16784 * t6552;
    let t82398 = F::cast_from(0.17544670867903938621e1_f64) * t5192 * t20384;
    let t82400 = F::cast_from(0.5848223622634646207e0_f64) * t3531 * t24498;
    let t82402 = F::cast_from(0.17544670867903938621e1_f64) * t20400 * t5202;
    (t82391, t82394, t82396, t82398, t82400, t82402)
}
