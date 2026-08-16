//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 175/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk175(t414: f64, t88: f64, t145: f64, t34: f64, t67: f64, t62: f64, t393: f64, t395: f64, t399: f64, t401: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t415 = t414 * t88;
    let t416 = 4.0_f64 * t415;
    let t417 = t34 * t145;
    let t433 = t67 * t67;
    let t434 = 1.0_f64 / t433;
    let t435 = t62 * t434;
    let t440 = -0.1176575e1_f64 * t393 - 0.516475e0_f64 * t395 - 0.2103875e0_f64 * t399 - 0.104195e0_f64 * t401;
    (t416, t417, t433, t434, t435, t440)
}
