//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 626/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk626<F: Float>(t5274: F, t285: F, t442: F, t5255: F, t441: F, t1114: F, t4573: F, sigma2: F) -> (F, F, F, F, F) {
    let t5275 = sigma2 * t5274;
    let t5276 = t5275 * t285;
    let t5279 = t442 * t5255;
    let t5280 = t441 * t5279;
    let t5285 = t1114 * t4573;
    (t5275, t5276, t5279, t5280, t5285)
}
