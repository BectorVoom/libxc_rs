//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 779/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk779(t1586: f64, t1993: f64, t1992: f64, t493: f64, t1450: f64, t1982: f64, t1981: f64, t3306: f64, t2065: f64, t435: f64, t132: f64, t2015: f64, t432: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5318 = t1993 * t1586;
    let t5319 = t1992 * t5318;
    let t5321 = t493 * t5319 / 15.0_f64;
    let t5322 = t1450 * t1982;
    let t5324 = 4.0_f64 / 45.0_f64 * t1981 * t5322;
    let t5325 = 2.0_f64 / 135.0_f64 * t3306;
    let t5326 = t435 * t2065;
    let t5328 = 2.0_f64 / 45.0_f64 * t132 * t5326;
    let t5330 = 2.0_f64 / 45.0_f64 * t432 * t2015;
    (t5318, t5319, t5321, t5322, t5324, t5325, t5326, t5328, t5330)
}
