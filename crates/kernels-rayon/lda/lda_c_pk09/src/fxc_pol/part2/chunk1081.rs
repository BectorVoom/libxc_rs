//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1081/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1081(t10959: f64, t11066: f64, t11073: f64, t11076: f64, t11529: f64, t11532: f64, t11535: f64, t11539: f64, t11542: f64, t6323: f64, t6337: f64, t6467: f64, t6508: f64, t6550: f64, t7183: f64, t7184: f64, t7188: f64) -> f64 {
    let t11820 = 0.15282509383508946_f64 * t11066 + 0.30565018767017893_f64 * t10959 + 1.532302805120685_f64 * t11529 - 1.532302805120685_f64 * t11532 - 1.532302805120685_f64 * t11535 + 2.2984542076810275_f64 * t11539 - 1.532302805120685_f64 * t11542 + 0.15282509383508946_f64 * t11076 + t7183 + 0.05094169794502982_f64 * t11073 + t7188 - 0.05094169794502982_f64 * t6337 - 0.15282509383508946_f64 * t6323 + 0.510767601706895_f64 * t6550 + t7184 - 0.510767601706895_f64 * t6508 + 0.05094169794502982_f64 * t6467;
    t11820
}
