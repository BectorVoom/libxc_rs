//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1186/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1186<F: Float>(t4844: F, t831: F, t161: F, t489: F, t6730: F, t432: F, t6227: F, t166: F, t2599: F, t9321: F, t14206: F, t14481: F, t14484: F, t17920: F, t17922: F, t17923: F, t17924: F, t17927: F, t17929: F, t17932: F, t17934: F) -> (F, F, F, F, F, F) {
    let t17935 = t831 * t4844;
    let t17936 = 2.0 / 135.0 * t17935;
    let t17938 = t161 * t489 * t6730;
    let t17939 = 4.0 / 45.0 * t17938;
    let t17941 = t432 * t6227 / 15.0;
    let t17945 = t161 * t166 * t9321 * t2599 / 15.0;
    let t17946 = 2.0 / 45.0 * t14206;
    let t17947 = 16.0 / 3.0 * t14481 + 8.0 / 3.0 * t14484 + t17920 + t17922 - t17923 - t17924 + t17927 - t17929 + t17932 - t17934 + t17936 - t17939 - t17941 + t17945 - t17946;
    (t17936, t17939, t17941, t17945, t17946, t17947)
}
