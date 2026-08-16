//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 165/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk165(t40: f64, t428: f64, t67: f64, t62: f64, t393: f64, t395: f64, t399: f64, t401: f64, t70: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t429 = t40 * t428;
    let t433 = t67 * t67;
    let t434 = 1.0_f64 / t433;
    let t435 = t62 * t434;
    let t440 = -0.1176575e1_f64 * t393 - 0.516475e0_f64 * t395 - 0.2103875e0_f64 * t399 - 0.104195e0_f64 * t401;
    let t441 = 1.0_f64 / t70;
    (t429, t433, t434, t435, t440, t441)
}
