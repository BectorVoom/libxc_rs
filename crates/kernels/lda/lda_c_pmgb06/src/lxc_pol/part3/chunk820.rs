//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 820/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk820<F: Float>(t2857: F, t432: F, t1441: F, t3213: F, t1423: F, t3191: F, t1431: F, t3220: F, t3255: F, t3263: F, t1179: F, t161: F, t165: F, t177: F, t2953: F, t3279: F, t464: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t10089 = t432 * t2857;
    let t10099 = t3213 * t1441;
    let t10101 = t1423 * t3191;
    let t10103 = t3220 * t1431;
    let t10105 = t3220 * t1441;
    let t10109 = t3213 * t1431;
    let t10111 = t1423 * t3255;
    let t10113 = t1423 * t3263;
    let t10134 = 28.0 / 1215.0 * t161 * t1179 * t165 * t177;
    let t10137 = t1423 * t2953;
    let t10148 = t3279 * t464;
    (t10089, t10099, t10101, t10103, t10105, t10109, t10111, t10113, t10134, t10137, t10148)
}
