//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1008/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1008(t2912: f64, t4884: f64, t1919: f64, t493: f64, t11964: f64, t11970: f64, t11972: f64, t11974: f64, t11977: f64, t11981: f64, t11985: f64, t11987: f64, t11990: f64, t9426: f64, t9429: f64) -> (f64, f64, f64) {
    let t11991 = t4884 * t2912;
    let t11994 = 4.0_f64 / 3.0_f64 * t493 * t1919 * t11991;
    let t11995 = t11964 + 8.0_f64 / 27.0_f64 * t9426 + t9429 - t11970 - t11972 + t11974 + t11977 - t11981 - t11985 + t11987 + t11990 + t11994;
    (t11991, t11994, t11995)
}
