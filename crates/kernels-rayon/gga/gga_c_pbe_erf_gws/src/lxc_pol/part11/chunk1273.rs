//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1273/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1273(t11412: f64, t20833: f64, t3791: f64, t8978: f64, t39388: f64, t46596: f64, t3134: f64, t46446: f64, t11778: f64, t11782: f64, t44230: f64, t1109: f64, t11994: f64, t13257: f64, t13263: f64, t13385: f64, t2253: f64, t2255: f64, t2266: f64, t2312: f64, t3258: f64, t3752: f64, t3772: f64, t3781: f64, t49932: f64, t904: f64, t916: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t50309 = t8978 * t20833 * t3791 * t11412 / 4.0_f64;
    let t50310 = 35.0_f64 / 36.0_f64 * t39388;
    let t50311 = 7.0_f64 / 72.0_f64 * t46596;
    let t50327 = t46446 * t3134 / 24.0_f64;
    let t50329 = t11782 * t11778 / 16.0_f64;
    let t50335 = t44230 * t3134 / 24.0_f64;
    let t50336 = 7.0_f64 / 512.0_f64 * t2266 * t916 * t904 * t49932 - t50309 + t50310 + t50311 + t2312 * t2255 * t11994 * t13257 / 48.0_f64 + t2312 * t2255 * t3258 * t13385 * t1109 / 48.0_f64 + t2312 * t2255 * t3258 * t3752 * t3772 / 96.0_f64 - t50327 - t50329 - t2253 * t2255 * t3781 * t13263 / 96.0_f64 - t50335;
    (t50309, t50310, t50311, t50327, t50329, t50335, t50336)
}
