//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 942/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk942(t5047: f64, t5071: f64, t5516: f64, t5529: f64, t5530: f64, t5535: f64, t5538: f64, t9628: f64, t9746: f64, t9753: f64, t9756: f64, t9922: f64, t9925: f64, t9929: f64, t9933: f64, t9936: f64, t9943: f64) -> f64 {
    let t9945 = -t5530 + t5535 + t5516 + t5529 + 0.04525483399593904_f64 * t5047 - t5538 + 0.015084944665313014_f64 * t5071 + 0.4537481858318121_f64 * t9922 - 0.4537481858318121_f64 * t9925 - 0.4537481858318121_f64 * t9929 + 0.6806222787477182_f64 * t9933 - 0.4537481858318121_f64 * t9936 + 0.04525483399593904_f64 * t9746 + 0.015084944665313014_f64 * t9753 + 0.04525483399593904_f64 * t9756 + 0.09050966799187808_f64 * t9628 - 0.15124939527727072_f64 * t9943;
    t9945
}
