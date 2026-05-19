//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1107/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1107<F: Float>(t11535: F, t10959: F, t11066: F, t11073: F, t11076: F, t11532: F, t11539: F, t11542: F, t12253: F, t6323: F, t6337: F, t6467: F, t6635: F, t6651: F, t7437: F, t7438: F, t7442: F) -> F {
    let t12255 = F::new(8.0) * t11535;
    let t12263 = F::cast_from(0.821419393556371_f64) * t11066 + F::cast_from(1.642838787112742_f64) * t10959 + t12253 - F::new(8.0) * t11532 - t12255 + F::new(12.0) * t11539 - F::new(8.0) * t11542 + F::cast_from(0.821419393556371_f64) * t11076 + t7437 + F::cast_from(0.2738064645187903_f64) * t11073 + t7442 - F::cast_from(0.2738064645187903_f64) * t6337 - F::cast_from(0.821419393556371_f64) * t6323 + t6651 + t7438 - t6635 + F::cast_from(0.2738064645187903_f64) * t6467;
    t12263
}
