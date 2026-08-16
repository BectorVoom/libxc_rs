//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1103/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1103(t4602: f64, t5322: f64, t1981: f64, t1982: f64, t3194: f64, t1592: f64, t1966: f64, t439: f64, t477: f64, t5039: f64, t9936: f64, t161: f64, t166: f64, t851: f64, t9603: f64) -> (f64, f64, f64, f64, f64) {
    let t13125 = 4.0_f64 / 15.0_f64 * t4602 * t5322;
    let t13128 = 2.0_f64 / 15.0_f64 * t1981 * t3194 * t1982;
    let t13133 = t439 * t1966 * t1592 * t5039 * t477 / 5.0_f64;
    let t13134 = t9936 / 45.0_f64;
    let t13138 = t161 * t166 * t9603 * t851 / 30.0_f64;
    (t13125, t13128, t13133, t13134, t13138)
}
