//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1366/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1366<F: Float>(t14206: F, t14481: F, t14484: F, t17920: F, t17922: F, t17923: F, t17924: F, t17927: F, t17929: F, t17932: F, t17934: F, t17936: F, t17939: F, t17941: F, t17945: F) -> (F, F) {
    let t17946 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t14206;
    let t17947 = F::cast_from(16.0_f64) / F::cast_from(3.0_f64) * t14481 + F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t14484 + t17920 + t17922 - t17923 - t17924 + t17927 - t17929 + t17932 - t17934 + t17936 - t17939 - t17941 + t17945 - t17946;
    (t17946, t17947)
}
