//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1275/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1275(t2168: f64, t4386: f64, t50354: f64, t1076: f64, t1123: f64, t1133: f64, t21640: f64, t21647: f64, t2253: f64, t2255: f64, t2343: f64, t3373: f64, t343: f64, t3854: f64, t50349: f64, t50353: f64, t50362: f64, t50363: f64, t50368: f64, t9665: f64) -> (f64, f64) {
    let t50371 = t2168 * t4386 * t50354 / 4.0_f64;
    let t50372 = -t2253 * t2255 * t1123 * t3373 * t1133 * t343 / 192.0_f64 - t2253 * t2255 * t1123 * t1076 * t3854 * t343 / 128.0_f64 + t21640 + t21647 + t50349 + t50353 + t2343 * t9665 * t50354 / 32.0_f64 + t50362 - t50363 + t50368 + t50371;
    (t50371, t50372)
}
