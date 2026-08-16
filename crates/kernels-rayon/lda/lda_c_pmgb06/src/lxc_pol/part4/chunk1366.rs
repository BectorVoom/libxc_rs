//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1366/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1366(t14206: f64, t14481: f64, t14484: f64, t17920: f64, t17922: f64, t17923: f64, t17924: f64, t17927: f64, t17929: f64, t17932: f64, t17934: f64, t17936: f64, t17939: f64, t17941: f64, t17945: f64) -> (f64, f64) {
    let t17946 = 2.0_f64 / 45.0_f64 * t14206;
    let t17947 = 16.0_f64 / 3.0_f64 * t14481 + 8.0_f64 / 3.0_f64 * t14484 + t17920 + t17922 - t17923 - t17924 + t17927 - t17929 + t17932 - t17934 + t17936 - t17939 - t17941 + t17945 - t17946;
    (t17946, t17947)
}
