//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 185/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk185<F: Float>(t493: F, t496: F, t155: F, t56: F, t174: F, t177: F, t188: F) -> (F, F, F, F, F) {
    let t498 = F::new(4.0) / F::new(15.0) * t493 * t496;
    let t499 = t155 * t56;
    let t501 = t174 * t499 * t177;
    let t502 = F::new(0.0018891666666666666) * t501;
    let t503 = t56 * t188;
    (t498, t499, t501, t502, t503)
}
