//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 172/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk172(t40: f64, t461: f64, t427: f64, t85: f64, t1: f64, t60: f64, t119: f64, t155: f64, t84: f64) -> (f64, f64, f64, f64) {
    let t462 = t40 * t461;
    let t463 = t427 * t85;
    let t464 = 0.19751789702565206229e-1_f64 * t463;
    let t465 = t60 * t1;
    let t467 = t119 * t155 * t84;
    (t462, t464, t465, t467)
}
