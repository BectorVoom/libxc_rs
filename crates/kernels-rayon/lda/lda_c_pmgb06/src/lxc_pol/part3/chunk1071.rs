//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1071/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1071(t1080: f64, t1464: f64, t2088: f64, t2991: f64, t493: f64, t10445: f64, t2911: f64, t2912: f64, t851: f64, t1894: f64, t3177: f64, t1420: f64, t5287: f64) -> (f64, f64, f64, f64) {
    let t12719 = t493 * t2991 * t2088 * t1464 * t1080 / 9.0_f64;
    let t12724 = 8.0_f64 / 81.0_f64 * t493 * t10445 * t851 * t2911 * t2912;
    let t12726 = t3177 * t1894 / 15.0_f64;
    let t12728 = 2.0_f64 / 15.0_f64 * t1420 * t5287;
    (t12719, t12724, t12726, t12728)
}
