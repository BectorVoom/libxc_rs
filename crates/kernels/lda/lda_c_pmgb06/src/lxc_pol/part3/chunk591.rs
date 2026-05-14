//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 591/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk591<F: Float>(t1271: F, t370: F, t97: F, t315: F, t342: F, t934: F, t109: F, t1227: F, t55: F, t1276: F, t1238: F, t56: F, t1243: F, t409: F, t110: F, t1263: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3566 = t1271 * t370 * t97;
    let t3568 = t934 * t315 * t342;
    let t3569 = t3566 * t3568;
    let t3572 = t55 * t109 * t1227;
    let t3573 = t1276 * t3572;
    let t3576 = t1238 * t56 * t97;
    let t3577 = t3576 * t3568;
    let t3578 = 0.9743416666666667 * t3577;
    let t3579 = t1243 * t3572;
    let t3580 = 1.4615125 * t3579;
    let t3582 = t55 * t409 * t342;
    let t3583 = t1276 * t3582;
    let t3585 = t110 * t1263;
    (t3566, t3569, t3573, t3576, t3577, t3578, t3579, t3580, t3582, t3583, t3585)
}
