//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 468/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk468<F: Float>(t115: F, t2341: F, t5: F, t363: F, t988: F, t992: F, t355: F, t287: F, t529: F, t362: F, t357: F, t984: F, t993: F, t176: F, t981: F, t998: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t2342 = t2341 * t115;
    let t2343 = t2342 * t5;
    let t2344 = t2343 * t363;
    let t2347 = t988 * t992;
    let t2348 = t355 * t2347;
    let t2350 = t529 * t287;
    let t2351 = t2350 * t362;
    let t2352 = t357 * t2351;
    let t2354 = t355 * t2352 / 9.0;
    let t2357 = t984 * t993;
    let t2359 = t176 * t981;
    let t2360 = t2359 * t998;
    (t2343, t2344, t2347, t2348, t2350, t2351, t2352, t2354, t2357, t2359, t2360)
}
