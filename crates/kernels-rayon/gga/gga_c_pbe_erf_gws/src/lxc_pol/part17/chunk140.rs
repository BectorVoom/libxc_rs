//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 140/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk140(t328: f64, t346: f64, t326: f64, t334: f64, t338: f64) -> (f64, f64, f64) {
    let t347 = t346 * t328;
    let t348 = t326 * t347;
    let t351 = t348 * t334 * t338 / 96.0_f64;
    let t352 = 1.0_f64 + t351;
    (t348, t351, t352)
}
