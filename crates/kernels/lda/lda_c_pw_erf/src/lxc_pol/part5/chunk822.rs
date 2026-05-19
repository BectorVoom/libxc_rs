//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 822/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk822<F: Float>(t4185: F, t4198: F, t4201: F, t4206: F, t4209: F, t4544: F, t4547: F, t4719: F, t7256: F, t7530: F, t7531: F, t7532: F, t7534: F, t7536: F, t7538: F, t7540: F, t7541: F) -> F {
    let t7544 = F::cast_from(0.21642082724729686_f64) * t4544 + F::cast_from(0.03354522822333102_f64) * t4547 - t4185 + t4198 + t4201 + t4206 - t4209 + t7530 - t7531 - t7532 - t7534 + t7536 + t7538 + t7540 - t7541 + F::new(4.0) * t4719 + F::new(4.0) * t7256;
    t7544
}
