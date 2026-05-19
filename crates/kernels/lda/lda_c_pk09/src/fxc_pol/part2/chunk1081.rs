//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1081/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1081<F: Float>(t10959: F, t11066: F, t11073: F, t11076: F, t11529: F, t11532: F, t11535: F, t11539: F, t11542: F, t6323: F, t6337: F, t6467: F, t6508: F, t6550: F, t7183: F, t7184: F, t7188: F) -> F {
    let t11820 = F::cast_from(0.15282509383508946_f64) * t11066 + F::cast_from(0.30565018767017893_f64) * t10959 + F::cast_from(1.532302805120685_f64) * t11529 - F::cast_from(1.532302805120685_f64) * t11532 - F::cast_from(1.532302805120685_f64) * t11535 + F::cast_from(2.2984542076810275_f64) * t11539 - F::cast_from(1.532302805120685_f64) * t11542 + F::cast_from(0.15282509383508946_f64) * t11076 + t7183 + F::cast_from(0.05094169794502982_f64) * t11073 + t7188 - F::cast_from(0.05094169794502982_f64) * t6337 - F::cast_from(0.15282509383508946_f64) * t6323 + F::cast_from(0.510767601706895_f64) * t6550 + t7184 - F::cast_from(0.510767601706895_f64) * t6508 + F::cast_from(0.05094169794502982_f64) * t6467;
    t11820
}
