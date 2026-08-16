//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 769/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk769(t2255: f64, t6298: f64, t2189: f64, t274: f64, t343: f64, t851: f64, t6: f64, t3235: f64, t875: f64, t2253: f64, t2343: f64, t3247: f64, t6246: f64, t6251: f64, t6255: f64, t6260: f64, t6262: f64, t6266: f64, t6273: f64, t6275: f64, t6279: f64, t6284: f64, t6289: f64, t6293: f64, t902: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6299 = t2255 * t6298;
    let t6303 = t274 * t2189 * t343;
    let t6304 = t851 * t6303;
    let t6305 = t2255 * t6304;
    let t6308 = t6 * t2189;
    let t6310 = t3235 * t6308 * t875;
    let t6313 = -t6246 + t6251 - t6255 - t6260 + t902 * t6262 / 768.0_f64 + t902 * t6266 / 1536.0_f64 + t6273 + t6275 * t6279 / 32.0_f64 + t2343 * t6284 / 128.0_f64 - 3.0_f64 / 128.0_f64 * t3247 * t6289 - t2253 * t6293 / 256.0_f64 - t2253 * t6299 / 256.0_f64 - t2253 * t6305 / 256.0_f64 - t2343 * t6310 / 512.0_f64;
    (t6299, t6303, t6305, t6308, t6310, t6313)
}
