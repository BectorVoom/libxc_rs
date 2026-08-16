//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 233/1404 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk233(t670: f64, t672: f64, t395: f64, t401: f64, t7: f64) -> (f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t674 = 0.10821041362364843377e0_f64 * t670 * t672;
    let t677 = 0.4125e0_f64 * t395 - t401 / 6.0_f64;
    let t678 = t677 * pi;
    let t679 = t678 * t7;
    (t674, t677, t678, t679)
}
