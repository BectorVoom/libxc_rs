//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 622/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk622<F: Float>(t1179: F, t3141: F, t462: F, t442: F, t2665: F, t446: F, t140: F, t3105: F, t3107: F, t1: F, t450: F, t3101: F, t438: F, t3138: F, t466: F, t429: F, t530: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3175 = t1179 * t3141;
    let t3181 = t462 * t462;
    let t3182 = 1.0 / t3181;
    let t3183 = t3182 * t442;
    let t3184 = t446 * t2665;
    let t3185 = t3184 * t140;
    let t3186 = t3183 * t3185;
    let t3187 = t3105 * t3107;
    let t3188 = t3187 * t1;
    let t3189 = t450 * t3188;
    let t3192 = t3101 * t3185;
    let t3194 = t3105 * t1 * t438;
    let t3195 = t450 * t3194;
    let t3199 = 0.16793568152788065763e-2 * t466 * t3138;
    let t3200 = t530 * t429;
    (t3175, t3181, t3182, t3183, t3186, t3187, t3188, t3189, t3192, t3194, t3195, t3199, t3200)
}
