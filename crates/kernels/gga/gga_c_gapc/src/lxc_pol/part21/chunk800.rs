//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 800/1125 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk800<F: Float>(t10293: F, t6943: F, t10292: F, t10264: F, t6179: F, t6182: F, t800: F, t10229: F, t6146: F, t2536: F, t10113: F, t876: F, t794: F, t188: F, t297: F, t818: F) -> (F, F, F, F, F, F) {
    let t10294 = t10293 * t6943;
    let t10295 = t10292 * t10294;
    let t10297 = t10264 * t6179;
    let t10298 = t800 * t6182;
    let t10299 = t10297 * t10298;
    let t10301 = t10229 * t6146;
    let t10302 = t10293 * t2536;
    let t10303 = t10301 * t10302;
    let t10305 = t10113 * t876;
    let t10306 = t794 * t10305;
    let t10309 = t188 * t818 * t297;
    (t10295, t10299, t10301, t10303, t10306, t10309)
}
