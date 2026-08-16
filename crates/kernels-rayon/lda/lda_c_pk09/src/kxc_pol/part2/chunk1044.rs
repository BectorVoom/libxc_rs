//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1044/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1044(t10954: f64, t10959: f64, t10962: f64, t10966: f64, t11062: f64, t11066: f64, t11070: f64, t11073: f64, t11076: f64, t6320: f64, t6323: f64, t6326: f64, t6327: f64, t6337: f64, t6465: f64, t6467: f64) -> f64 {
    let t11351 = t6320 - 2.0_f64 * t6323 + t6326 + 2.0_f64 * t6327 - 2.0_f64 * t10954 + 4.0_f64 * t10959 - 2.0_f64 / 3.0_f64 * t10962 - 2.0_f64 * t10966 - 2.0_f64 * t11062 - 2.0_f64 / 3.0_f64 * t6337 - t6465 + 2.0_f64 / 3.0_f64 * t6467 + 2.0_f64 * t11066 - 2.0_f64 * t11070 + 2.0_f64 / 3.0_f64 * t11073 + 2.0_f64 * t11076;
    t11351
}
