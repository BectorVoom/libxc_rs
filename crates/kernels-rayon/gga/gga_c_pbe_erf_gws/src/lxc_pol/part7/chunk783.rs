//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 783/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk783(t2300: f64, t6449: f64, t904: f64, t2331: f64, t899: f64, t912: f64, t918: f64, t2210: f64, t858: f64, t884: f64, t2253: f64, t2277: f64, t2312: f64, t6387: f64, t6392: f64, t6398: f64, t6403: f64, t6406: f64, t6413: f64, t6415: f64, t6417: f64, t6444: f64, t6446: f64, t6448: f64, t929: f64) -> (f64, f64, f64, f64, f64) {
    let t6451 = t2300 * t904 * t6449;
    let t6455 = t899 * t912 * t2331;
    let t6456 = t6455 * t918;
    let t6459 = t2210 * t858 * t6449;
    let t6461 = 3.0_f64 / 16.0_f64 * t884 * t6459;
    let t6462 = -5.0_f64 / 128.0_f64 * t929 * t6387 + t2312 * t6392 / 128.0_f64 - t2253 * t6398 / 256.0_f64 + 7.0_f64 / 192.0_f64 * t6403 - t2277 * t6406 / 768.0_f64 - t6413 - t6415 + 7.0_f64 / 768.0_f64 * t6417 - t6444 + t6446 + t6448 + 5.0_f64 / 256.0_f64 * t929 * t6451 - 119.0_f64 / 2304.0_f64 * t6456 + t6461;
    (t6451, t6455, t6459, t6461, t6462)
}
