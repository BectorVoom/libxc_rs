//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 853/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk853(t69: f64, t8357: f64, t1035: f64, t1041: f64, t1043: f64, t3947: f64, t687: f64, t217: f64, t219: f64, t1024: f64, t633: f64, t3952: f64, t654: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8439 = t69 * t8357;
    let t8479 = t1035 * t1035;
    let t8482 = 48.245938496077606_f64 * t1041 * t8479 * t1043;
    let t8483 = t3947 * t687;
    let t8485 = 1.0_f64 / t217;
    let t8499 = 1.0_f64 / t219;
    let t8519 = 6.0_f64 * t1024 * t8479 * t633;
    let t8520 = t3952 * t654;
    (t8439, t8482, t8483, t8485, t8499, t8519, t8520)
}
