//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 832/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk832<F: Float>(t7024: F, t7081: F, t7135: F, t7181: F, t7322: F, t7457: F, t7520: F, t7585: F, t7630: F, t7683: F, t7738: F, t7768: F, t7802: F, t7861: F, t7924: F, t7972: F) -> (F,) {
    let t7976 = t7024 + t7081 + t7135 + t7181 + t7322 + t7457 + t7520 + t7585 + t7630 + t7683 + t7738 + t7768 + t7802 + t7861 + t7924 + t7972;
    (t7976,)
}
