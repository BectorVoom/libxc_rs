//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 182/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk182<F: Float>(t482: F, t483: F, t485: F, t163: F, t169: F, t234: F, t299: F, t172: F, t181: F, t184: F) -> (F, F, F, F) {
    let t487 = F::cast_from(0.001975389032890948_f64) * t482 * t483 * t485;
    let t491 = F::cast_from(0.008980675507690957_f64) * t169 * t299 * t234 * t163;
    let t492 = t172 * t181;
    let t493 = t492 * t184;
    (t487, t491, t492, t493)
}
