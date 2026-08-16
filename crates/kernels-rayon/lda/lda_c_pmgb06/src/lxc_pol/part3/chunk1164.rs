//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1164/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1164(t5442: f64, t5499: f64, t1916: f64, t3226: f64, t1447: f64, t5448: f64, t2979: f64, t493: f64, t5358: f64, t1080: f64, t1380: f64, t1414: f64, t2088: f64) -> (f64, f64, f64, f64, f64) {
    let t13891 = t5499 * t5442;
    let t13892 = 2.0_f64 / 9.0_f64 * t13891;
    let t13893 = t3226 * t1916;
    let t13894 = 8.0_f64 / 45.0_f64 * t13893;
    let t13895 = t1447 * t5448;
    let t13896 = 8.0_f64 / 45.0_f64 * t13895;
    let t13899 = 2.0_f64 / 15.0_f64 * t493 * t2979 * t5358;
    let t13904 = 2.0_f64 / 15.0_f64 * t493 * t1380 * t2088 * t1414 * t1080;
    (t13892, t13894, t13896, t13899, t13904)
}
