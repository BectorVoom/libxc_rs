//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 854/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk854(t1289: f64, t342: f64, t4232: f64, t1311: f64, t26: f64, t329: f64, t1035: f64, t1041: f64, t1043: f64, t3947: f64, t687: f64, t217: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8470 = t4232 * t1289 * t342;
    let t8473 = t26 * t1311;
    let t8474 = t329 * t8473;
    let t8479 = t1035 * t1035;
    let t8482 = 48.245938496077606_f64 * t1041 * t8479 * t1043;
    let t8483 = t3947 * t687;
    let t8485 = 1.0_f64 / t217;
    (t8470, t8473, t8474, t8479, t8482, t8483, t8485)
}
