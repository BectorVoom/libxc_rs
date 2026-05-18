//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 888/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk888<F: Float>(t3317: F, t3319: F, t3335: F, t3342: F, t3384: F, t3388: F, t3393: F, t4231: F, t4234: F, t4235: F, t4236: F, t7851: F, t7855: F) -> F {
    let t9363 = F::new(4.59690841536205) * t7851 + F::new(4.59690841536205) * t7855 - F::new(0.3056501876701794) * t3335 - F::new(0.2037667917801196) * t3342 + F::new(9.1938168307241) * t3384 + F::new(9.1938168307241) * t3388 - F::new(9.1938168307241) * t3393 + t4231 + t4234 + t4235 - t4236 + F::new(0.3056501876701794) * t3317 + F::new(0.3056501876701794) * t3319;
    t9363
}
