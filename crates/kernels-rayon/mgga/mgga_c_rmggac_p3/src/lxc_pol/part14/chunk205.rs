//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 205/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk205(t306: f64, t84: f64, t89: f64, t312: f64, t263: f64, rho0: f64, tau0: f64) -> (f64, f64, f64, f64, f64) {
    let t801 = t306 * rho0;
    let t803 = 1.0_f64 / t84 / t801;
    let t804 = tau0 * t803;
    let t809 = 1.0_f64 / t89;
    let t810 = t312 * t312;
    let t811 = t809 * t810;
    let t814 = 1.0_f64 / t263;
    (t804, t809, t810, t811, t814)
}
