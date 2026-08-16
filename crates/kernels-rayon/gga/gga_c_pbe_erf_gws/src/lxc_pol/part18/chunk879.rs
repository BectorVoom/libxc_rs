//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 879/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk879(t810: f64, t824: f64, t2118: f64, t814: f64, t3224: f64, t6402: f64, t3287: f64, t6203: f64, t3232: f64, t6627: f64, t3237: f64, t2289: f64, t3283: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9504 = t824 * t810;
    let t9520 = t2118 * t814;
    let t9539 = 7.0_f64 / 576.0_f64 * t6402 * t3224;
    let t9549 = 7.0_f64 / 288.0_f64 * t6203 * t3287;
    let t9565 = 7.0_f64 / 288.0_f64 * t6627 * t3232;
    let t9579 = 7.0_f64 / 1152.0_f64 * t6627 * t3237;
    let t9592 = 7.0_f64 / 1152.0_f64 * t2289 * t3283;
    (t9504, t9520, t9539, t9549, t9565, t9579, t9592)
}
