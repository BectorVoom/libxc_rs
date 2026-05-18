//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1203/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1203<F: Float>(t14103: F, t14105: F, t14108: F, t14112: F, t14157: F, t14162: F, t14164: F, t14166: F, t14170: F, t14174: F, t14176: F, t14178: F, t14183: F) -> F {
    let t14184 = F::new(0.03354522822333102) * t14103 - F::new(0.011181742741110338) * t14105 + t14108 + t14112 + t14157 - t14162 + t14164 + t14166 + t14170 - t14174 + t14176 + t14178 + t14183;
    t14184
}
