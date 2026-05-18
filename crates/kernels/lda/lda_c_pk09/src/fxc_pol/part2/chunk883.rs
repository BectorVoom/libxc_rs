//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 883/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk883<F: Float>(t3317: F, t3319: F, t3335: F, t3342: F, t3384: F, t3388: F, t3393: F, t4187: F, t4190: F, t4191: F, t4192: F, t7851: F, t7855: F) -> F {
    let t9301 = F::new(4.431130547644593) * t7851 + F::new(4.431130547644593) * t7855 - F::new(0.2946275542389858) * t3335 - F::new(0.1964183694926572) * t3342 + F::new(8.862261095289186) * t3384 + F::new(8.862261095289186) * t3388 - F::new(8.862261095289186) * t3393 + t4187 + t4190 + t4191 - t4192 + F::new(0.2946275542389858) * t3317 + F::new(0.2946275542389858) * t3319;
    t9301
}
