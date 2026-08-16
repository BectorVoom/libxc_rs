//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3133/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3133(t24252: f64, t300: f64, t1198: f64, t1765: f64, t68609: f64, t16784: f64, t6552: f64, t20384: f64, t5192: f64, t24498: f64, t3531: f64, t20400: f64, t5202: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t82389 = t300 * t24252;
    let t82391 = 0.5848223622634646207e0_f64 * t82389 * t1198;
    let t82394 = 0.17544670867903938621e1_f64 * t68609 * t1765;
    let t82396 = 0.17544670867903938621e1_f64 * t16784 * t6552;
    let t82398 = 0.17544670867903938621e1_f64 * t5192 * t20384;
    let t82400 = 0.5848223622634646207e0_f64 * t3531 * t24498;
    let t82402 = 0.17544670867903938621e1_f64 * t20400 * t5202;
    (t82391, t82394, t82396, t82398, t82400, t82402)
}
