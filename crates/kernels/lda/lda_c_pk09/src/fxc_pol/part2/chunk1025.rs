//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1025/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1025<F: Float>(t10954: F, t10959: F, t10962: F, t10966: F, t11062: F, t11066: F, t11070: F, t11073: F, t11076: F, t6323: F, t6327: F, t6337: F, t6467: F, t7537: F, t7539: F, t7545: F) -> F {
    let t11078 = t7537 - F::new(1.5625) * t6323 + t7539 + F::new(1.5625) * t6327 - F::new(1.5625) * t10954 + F::new(3.125) * t10959 - F::new(0.5208333333333334) * t10962 - F::new(1.5625) * t10966 - F::new(1.5625) * t11062 - F::new(0.5208333333333334) * t6337 - t7545 + F::new(0.5208333333333334) * t6467 + F::new(1.5625) * t11066 - F::new(1.5625) * t11070 + F::new(0.5208333333333334) * t11073 + F::new(1.5625) * t11076;
    t11078
}
