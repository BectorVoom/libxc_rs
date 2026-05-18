//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 770/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk770<F: Float>(t3317: F, t3319: F, t3335: F, t3342: F, t3384: F, t3388: F, t3393: F, t3398: F, t3410: F, t3411: F, t3412: F, t7851: F, t7855: F) -> F {
    let t7864 = F::new(18.75) * t7851 + F::new(18.75) * t7855 - F::new(1.2466946262544771) * t3335 - F::new(0.8311297508363181) * t3342 + F::new(37.5) * t3384 + F::new(37.5) * t3388 - F::new(37.5) * t3393 + t3398 + t3410 + t3411 - t3412 + F::new(1.2466946262544771) * t3317 + F::new(1.2466946262544771) * t3319;
    t7864
}
