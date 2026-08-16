//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1427/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1427(t449: f64, t78211: f64, t78223: f64, t300: f64, t14850: f64, t21724: f64, t1118: f64, t11190: f64, t78129: f64, t6020: f64, t3264: f64, t3313: f64, t3315: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t78225 = (t78211 + t78223) * t449;
    let t78227 = 0.19751673498613801407e-1_f64 * t300 * t78225;
    let t78229 = 24.0_f64 * t14850 * t21724;
    let t78232 = 24.0_f64 * t11190 * t78129 * t1118;
    let t78233 = t6020 * t6020;
    let t78236 = 6.0_f64 * t3264 * t78233 * t1118;
    let t78239 = 0.48245938496077605201e2_f64 * t3313 * t78233 * t3315;
    (t78225, t78227, t78229, t78232, t78236, t78239)
}
