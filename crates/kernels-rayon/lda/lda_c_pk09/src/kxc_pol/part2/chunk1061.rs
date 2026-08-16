//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1061/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1061(t10959: f64, t11066: f64, t11073: f64, t11076: f64, t11529: f64, t11532: f64, t11535: f64, t11539: f64, t11542: f64, t6323: f64, t6337: f64, t6467: f64, t6508: f64, t6550: f64, t7149: f64, t7150: f64, t7154: f64) -> f64 {
    let t11551 = 0.2946275542389858_f64 * t11066 + 0.5892551084779716_f64 * t10959 + 2.9540870317630623_f64 * t11529 - 2.9540870317630623_f64 * t11532 - 2.9540870317630623_f64 * t11535 + 4.431130547644593_f64 * t11539 - 2.9540870317630623_f64 * t11542 + 0.2946275542389858_f64 * t11076 + t7149 + 0.0982091847463286_f64 * t11073 + t7154 - 0.0982091847463286_f64 * t6337 - 0.2946275542389858_f64 * t6323 + 0.9846956772543541_f64 * t6550 + t7150 - 0.9846956772543541_f64 * t6508 + 0.0982091847463286_f64 * t6467;
    t11551
}
