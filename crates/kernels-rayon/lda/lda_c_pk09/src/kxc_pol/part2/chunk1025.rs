//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1025/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1025(t10954: f64, t10959: f64, t10962: f64, t10966: f64, t11062: f64, t11066: f64, t11070: f64, t11073: f64, t11076: f64, t6323: f64, t6327: f64, t6337: f64, t6467: f64, t7537: f64, t7539: f64, t7545: f64) -> f64 {
    let t11078 = t7537 - 1.5625_f64 * t6323 + t7539 + 1.5625_f64 * t6327 - 1.5625_f64 * t10954 + 3.125_f64 * t10959 - 0.5208333333333334_f64 * t10962 - 1.5625_f64 * t10966 - 1.5625_f64 * t11062 - 0.5208333333333334_f64 * t6337 - t7545 + 0.5208333333333334_f64 * t6467 + 1.5625_f64 * t11066 - 1.5625_f64 * t11070 + 0.5208333333333334_f64 * t11073 + 1.5625_f64 * t11076;
    t11078
}
