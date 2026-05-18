//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 970/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk970<F: Float>(t5043: F, t5056: F, t5158: F, t5166: F, t5177: F, t5193: F, t5907: F, t5923: F, t9623: F, t9631: F, t9635: F, t9742: F, t9750: F, t9948: F, t9952: F, t9956: F, t9959: F) -> F {
    let t10330 = F::new(1.5323028051206833) * t9948 + F::new(1.5323028051206833) * t9952 - F::new(1.5323028051206833) * t9956 + F::new(1.0215352034137888) * t9959 - F::new(0.3056501876701794) * t9623 - F::new(0.1018833958900598) * t9631 - F::new(0.3056501876701794) * t9635 - F::new(0.3056501876701794) * t9742 - F::new(0.3056501876701794) * t9750 - F::new(0.3056501876701794) * t5043 - F::new(0.1018833958900598) * t5056 + t5907 - F::new(1.0215352034137888) * t5177 + F::new(1.0215352034137888) * t5193 + t5923 - F::new(3.0646056102413666) * t5158 + F::new(3.0646056102413666) * t5166;
    t10330
}
