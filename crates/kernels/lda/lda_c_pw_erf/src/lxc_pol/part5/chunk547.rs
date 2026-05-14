//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 547/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk547<F: Float>(t125: F, t917: F, t128: F, t2: F, t39: F, t2715: F, t103: F, t1710: F, t440: F, t442: F, t131: F, t1125: F, t120: F, t133: F, t3227: F, t153: F, t274: F, t2869: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3309 = t125 * t917;
    let t3310 = t128 * t2;
    let t3313 = 0.3264533333333333 * t3309 * t3310 * t39;
    let t3318 = param_hyb_omega_0 * t2715;
    let t3319 = t103 * t2;
    let t3322 = 1.9486833333333333 * t3318 * t3319 * t39;
    let t3332 = t440 * t1710;
    let t3337 = t442 * t442;
    let t3338 = 1.0 / t3337;
    let t3339 = t131 * t3338;
    let t3348 = 0.8940581481481481 * t133 * t1125 * t120;
    let t3349 = t133 * t3227;
    let t3373 = 4.429070076315393 * t153 * t2869 * t274;
    (t3309, t3310, t3313, t3318, t3319, t3322, t3332, t3337, t3338, t3339, t3348, t3349, t3373)
}
