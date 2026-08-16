//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1087/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1087(t2812: f64, t7248: f64, t10959: f64, t11066: f64, t11073: f64, t11076: f64, t11529: f64, t11532: f64, t11535: f64, t11539: f64, t11542: f64, t6323: f64, t6337: f64, t6467: f64, t6508: f64, t6550: f64, t6873: f64, t6874: f64, t6878: f64) -> (f64, f64) {
    let t11915 = t2812 * t7248;
    let t11936 = 0.04525483399593904_f64 * t11066 + 0.09050966799187808_f64 * t10959 + 0.4537481858318121_f64 * t11529 - 0.4537481858318121_f64 * t11532 - 0.4537481858318121_f64 * t11535 + 0.6806222787477182_f64 * t11539 - 0.4537481858318121_f64 * t11542 + 0.04525483399593904_f64 * t11076 + t6873 + 0.015084944665313014_f64 * t11073 + t6878 - 0.015084944665313014_f64 * t6337 - 0.04525483399593904_f64 * t6323 + 0.15124939527727072_f64 * t6550 + t6874 - 0.15124939527727072_f64 * t6508 + 0.015084944665313014_f64 * t6467;
    (t11915, t11936)
}
