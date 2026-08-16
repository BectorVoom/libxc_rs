//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1158/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1158(t1447: f64, t6783: f64, t1080: f64, t1380: f64, t1414: f64, t2623: f64, t493: f64, t1925: f64, t5194: f64, t1972: f64, t5359: f64, t1423: f64, t6788: f64) -> (f64, f64, f64, f64, f64) {
    let t15237 = t1447 * t6783;
    let t15238 = 4.0_f64 / 135.0_f64 * t15237;
    let t15243 = 2.0_f64 / 45.0_f64 * t493 * t1380 * t2623 * t1414 * t1080;
    let t15244 = t5194 * t1925;
    let t15245 = 8.0_f64 / 135.0_f64 * t15244;
    let t15247 = 4.0_f64 / 45.0_f64 * t1972 * t5359;
    let t15248 = t1423 * t6788;
    (t15238, t15243, t15245, t15247, t15248)
}
