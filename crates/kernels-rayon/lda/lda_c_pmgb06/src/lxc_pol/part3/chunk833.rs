//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 833/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk833(t5566: f64, t5595: f64, t5898: f64, t5913: f64, t23: f64, t342: f64, t24: f64, t5582: f64, t4042: f64, t73: f64, t1233: f64, t165: f64, t842: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5915 = t5566 + t5595 + t5898 + t5913;
    let t5939 = t342 * t23;
    let t6006 = t24 * t5582;
    let t6007 = t4042 * t73;
    let t6018 = t1233 * t5582;
    let t6119 = t165 * t842;
    (t5915, t5939, t6006, t6007, t6018, t6119)
}
