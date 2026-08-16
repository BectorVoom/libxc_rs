//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1153/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1153(t15184: f64, t493: f64, t5358: f64, t5486: f64, t1423: f64, t6775: f64, t1069: f64, t1385: f64, t1531: f64, t2648: f64, t439: f64, t1908: f64, t5220: f64) -> (f64, f64, f64, f64, f64) {
    let t15185 = 8.0_f64 / 135.0_f64 * t15184;
    let t15188 = 4.0_f64 / 45.0_f64 * t493 * t5486 * t5358;
    let t15189 = t1423 * t6775;
    let t15190 = 4.0_f64 / 135.0_f64 * t15189;
    let t15195 = 2.0_f64 / 45.0_f64 * t439 * t1385 * t2648 * t1531 * t1069;
    let t15196 = t5220 * t1908;
    (t15185, t15188, t15190, t15195, t15196)
}
