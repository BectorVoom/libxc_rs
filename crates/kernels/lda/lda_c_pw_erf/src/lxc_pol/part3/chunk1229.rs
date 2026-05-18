//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1229/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1229<F: Float>(t14414: F, t14415: F, t14417: F, t14418: F, t14421: F, t14427: F, t14438: F, t14451: F, t2765: F, t411: F, t4429: F, t5677: F, t684: F) -> (F, F, F) {
    let t14454 = t14414 + t14415 + t14417 + t14418 + t14421 + t14427 + t14438 + t14451;
    let t14465 = t2765 * t4429 * t411;
    let t14468 = t684 * t5677;
    (t14454, t14465, t14468)
}
