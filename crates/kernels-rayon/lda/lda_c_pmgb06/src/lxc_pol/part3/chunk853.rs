//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 853/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk853(t69: f64, t8315: f64, t8381: f64, t8378: f64, t8357: f64, t8312: f64, t2247: f64, t3650: f64, t5858: f64, t1289: f64, t374: f64, t6007: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8431 = t69 * t8315;
    let t8433 = t69 * t8381;
    let t8435 = t69 * t8378;
    let t8439 = t69 * t8357;
    let t8441 = t69 * t8312;
    let t8455 = t2247 * t5858 * t3650;
    let t8466 = t6007 * t1289 * t374;
    (t8431, t8433, t8435, t8439, t8441, t8455, t8466)
}
