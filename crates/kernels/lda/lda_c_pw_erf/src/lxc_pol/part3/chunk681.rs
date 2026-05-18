//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 681/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk681<F: Float>(t3562: F, t3567: F, t3571: F, t3573: F, t3575: F, t3578: F, t3659: F, t3662: F, t3665: F, t3673: F, t3681: F, t3708: F, t3711: F, t3713: F, t3718: F, t3720: F, t3726: F) -> F {
    let t4179 = t3562 + t3567 - t3571 + t3573 - t3575 - t3578 - t3659 - t3662 + t3665 - t3673 - t3681 + t3708 + t3711 - t3713 - t3718 - t3720 - t3726;
    t4179
}
