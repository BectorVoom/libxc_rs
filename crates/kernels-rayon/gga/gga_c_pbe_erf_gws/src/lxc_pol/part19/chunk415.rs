//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 415/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk415(t1513: f64, t1515: f64, t1243: f64, t486: f64, t48: f64, t53: f64, t118: f64, t119: f64, t120: f64, t331: f64, t156: f64, t497: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1516 = t1513 * t1515;
    let t1519 = 0.64956111111111111111e0_f64 * t486 * t1243;
    let t1523 = 1.0_f64 / t48;
    let t1528 = 1.0_f64 / t53;
    let t1540 = t118 * t119 * t331 * t120 / 9.0_f64;
    let t1541 = t156 * t497;
    (t1516, t1519, t1523, t1528, t1540, t1541)
}
