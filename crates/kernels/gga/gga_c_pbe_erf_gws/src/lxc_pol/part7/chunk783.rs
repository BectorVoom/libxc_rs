//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 783/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk783<F: Float>(t2300: F, t6449: F, t904: F, t2331: F, t899: F, t912: F, t918: F, t2210: F, t858: F, t884: F, t2253: F, t2277: F, t2312: F, t6387: F, t6392: F, t6398: F, t6403: F, t6406: F, t6413: F, t6415: F, t6417: F, t6444: F, t6446: F, t6448: F, t929: F) -> (F, F, F, F, F) {
    let t6451 = t2300 * t904 * t6449;
    let t6455 = t899 * t912 * t2331;
    let t6456 = t6455 * t918;
    let t6459 = t2210 * t858 * t6449;
    let t6461 = F::new(3.0) / F::new(16.0) * t884 * t6459;
    let t6462 = -F::new(5.0) / F::new(128.0) * t929 * t6387 + t2312 * t6392 / F::new(128.0) - t2253 * t6398 / F::new(256.0) + F::new(7.0) / F::new(192.0) * t6403 - t2277 * t6406 / F::new(768.0) - t6413 - t6415 + F::new(7.0) / F::new(768.0) * t6417 - t6444 + t6446 + t6448 + F::new(5.0) / F::new(256.0) * t929 * t6451 - F::new(119.0) / F::new(2304.0) * t6456 + t6461;
    (t6451, t6455, t6459, t6461, t6462)
}
