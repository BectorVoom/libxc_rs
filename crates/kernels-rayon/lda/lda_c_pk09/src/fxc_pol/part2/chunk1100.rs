//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1100/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1100(t10954: f64, t10959: f64, t10962: f64, t10966: f64, t11062: f64, t11066: f64, t11070: f64, t11073: f64, t11076: f64, t6323: f64, t6327: f64, t6337: f64, t6467: f64, t6836: f64, t6838: f64, t6844: f64) -> f64 {
    let t12133 = t6836 - 0.9421211958699838_f64 * t6323 + t6838 + 0.9421211958699838_f64 * t6327 - 0.9421211958699838_f64 * t10954 + 1.8842423917399675_f64 * t10959 - 0.3140403986233279_f64 * t10962 - 0.9421211958699838_f64 * t10966 - 0.9421211958699838_f64 * t11062 - 0.3140403986233279_f64 * t6337 - t6844 + 0.3140403986233279_f64 * t6467 + 0.9421211958699838_f64 * t11066 - 0.9421211958699838_f64 * t11070 + 0.3140403986233279_f64 * t11073 + 0.9421211958699838_f64 * t11076;
    t12133
}
