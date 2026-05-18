//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 847/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk847<F: Float>(t109: F, t3559: F, t55: F, t1243: F, t1267: F, t410: F, t360: F, t110: F, t3560: F, t1227: F, t315: F, t934: F) -> (F, F, F, F, F, F, F) {
    let t8309 = t55 * t109 * t3559;
    let t8310 = t1243 * t8309;
    let t8312 = t410 * t1267;
    let t8313 = t360 * t8312;
    let t8315 = t110 * t3560;
    let t8316 = t360 * t8315;
    let t8323 = t934 * t315 * t1227;
    (t8309, t8310, t8312, t8313, t8315, t8316, t8323)
}
