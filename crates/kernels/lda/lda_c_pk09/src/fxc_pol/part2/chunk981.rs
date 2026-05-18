//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 981/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk981<F: Float>(t5043: F, t5056: F, t5159: F, t5167: F, t5178: F, t5194: F, t5352: F, t5368: F, t9623: F, t9631: F, t9635: F, t9742: F, t9750: F, t9948: F, t9952: F, t9956: F, t9959: F) -> F {
    let t10501 = F::new(4.0) * t9948 + F::new(4.0) * t9952 - F::new(4.0) * t9956 + F::new(2.6666666666666665) * t9959 - F::new(0.821419393556371) * t9623 - F::new(0.2738064645187903) * t9631 - F::new(0.821419393556371) * t9635 - F::new(0.821419393556371) * t9742 - F::new(0.821419393556371) * t9750 - F::new(0.821419393556371) * t5043 - F::new(0.2738064645187903) * t5056 + t5352 - t5178 + t5194 + t5368 - t5159 + t5167;
    t10501
}
