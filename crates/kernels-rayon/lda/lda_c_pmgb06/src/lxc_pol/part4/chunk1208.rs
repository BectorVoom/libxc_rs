//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1208/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1208(t1080: f64, t1464: f64, t2623: f64, t2991: f64, t493: f64, t1972: f64, t5277: f64, t5282: f64, t439: f64, t5267: f64, t5482: f64, t1074: f64, t1385: f64, t6773: f64) -> (f64, f64, f64, f64, f64) {
    let t15923 = t493 * t2991 * t2623 * t1464 * t1080 / 27.0_f64;
    let t15925 = 2.0_f64 / 45.0_f64 * t1972 * t5277;
    let t15927 = 2.0_f64 / 27.0_f64 * t1972 * t5282;
    let t15930 = 2.0_f64 / 45.0_f64 * t439 * t5482 * t5267;
    let t15934 = t439 * t1385 * t6773 * t1074 / 45.0_f64;
    (t15923, t15925, t15927, t15930, t15934)
}
