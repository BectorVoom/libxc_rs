//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1031/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1031(t1758: f64, t534: f64, t6433: f64, t2: f64, t41: f64, t14: f64, t209: f64, t6567: f64, t543: f64, t6363: f64, t133: f64, t1765: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t22494 = t1758 * t1758;
    let t22497 = 24.0_f64 * t6433 * t22494 * t534;
    let t22502 = t2 * t41;
    let t22508 = 1.0_f64 / t14 / t22502 * t2 * t6567 * t209 / 48.0_f64;
    let t22510 = t6363 * t543;
    let t22512 = t1765 * t133;
    (t22494, t22497, t22502, t22508, t22510, t22512)
}
