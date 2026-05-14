//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 899/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk899<F: Float>(t18751: F, t69: F, t18754: F, t18744: F, t11475: F, t2247: F, t7073: F, t5858: F, t7077: F, t6961: F, t707: F, t6957: F, t2448: F, t384: F, t1147: F, t123: F, t2407: F, t317: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18829 = t69 * t18751;
    let t18831 = t69 * t18754;
    let t18837 = t69 * t18744;
    let t18848 = t2247 * t11475 * t7073;
    let t18851 = t2247 * t5858 * t7077;
    let t18883 = t707 * t6961;
    let t18885 = t707 * t6957;
    let t18903 = t384 * t2448;
    let t18911 = t123 * t1147 * t2407 * t317;
    (t18829, t18831, t18837, t18848, t18851, t18883, t18885, t18903, t18911)
}
