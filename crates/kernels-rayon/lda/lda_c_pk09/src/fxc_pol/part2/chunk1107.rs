//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1107/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1107(t11535: f64, t10959: f64, t11066: f64, t11073: f64, t11076: f64, t11532: f64, t11539: f64, t11542: f64, t12253: f64, t6323: f64, t6337: f64, t6467: f64, t6635: f64, t6651: f64, t7437: f64, t7438: f64, t7442: f64) -> f64 {
    let t12255 = 8.0_f64 * t11535;
    let t12263 = 0.821419393556371_f64 * t11066 + 1.642838787112742_f64 * t10959 + t12253 - 8.0_f64 * t11532 - t12255 + 12.0_f64 * t11539 - 8.0_f64 * t11542 + 0.821419393556371_f64 * t11076 + t7437 + 0.2738064645187903_f64 * t11073 + t7442 - 0.2738064645187903_f64 * t6337 - 0.821419393556371_f64 * t6323 + t6651 + t7438 - t6635 + 0.2738064645187903_f64 * t6467;
    t12263
}
