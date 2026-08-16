//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1087/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1087<F: Float>(t2812: F, t7248: F, t10959: F, t11066: F, t11073: F, t11076: F, t11529: F, t11532: F, t11535: F, t11539: F, t11542: F, t6323: F, t6337: F, t6467: F, t6508: F, t6550: F, t6873: F, t6874: F, t6878: F) -> (F, F) {
    let t11915 = t2812 * t7248;
    let t11936 = F::cast_from(0.04525483399593904_f64) * t11066 + F::cast_from(0.09050966799187808_f64) * t10959 + F::cast_from(0.4537481858318121_f64) * t11529 - F::cast_from(0.4537481858318121_f64) * t11532 - F::cast_from(0.4537481858318121_f64) * t11535 + F::cast_from(0.6806222787477182_f64) * t11539 - F::cast_from(0.4537481858318121_f64) * t11542 + F::cast_from(0.04525483399593904_f64) * t11076 + t6873 + F::cast_from(0.015084944665313014_f64) * t11073 + t6878 - F::cast_from(0.015084944665313014_f64) * t6337 - F::cast_from(0.04525483399593904_f64) * t6323 + F::cast_from(0.15124939527727072_f64) * t6550 + t6874 - F::cast_from(0.15124939527727072_f64) * t6508 + F::cast_from(0.015084944665313014_f64) * t6467;
    (t11915, t11936)
}
