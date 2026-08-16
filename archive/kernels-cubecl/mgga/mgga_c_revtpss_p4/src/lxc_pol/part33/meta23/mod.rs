//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta23 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk161;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk162;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk163;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk164;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk165;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk166;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta23<F: Float>(t406: F, t409: F, t412: F, t416: F, t439: F, t300: F, t424: F, t426: F, t435: F, t344: F, t56: F, t404: F, t221: F, t65: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t444, t447, t448) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk161::<F>(t406, t409, t412, t416);
        let (t452, t454, t456) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk162::<F>(t439, t448, t300, t424, t426, t435, t406);
        let (t458, t459) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk163::<F>(t406);
        let t460 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk164::<F>(t456, t459);
        let (t461, t462) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk165::<F>(t344, t56, t404);
        let t464 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk166::<F>(t221, t462, t65);
    (t444, t447, t448, t452, t454, t456, t458, t459, t460, t461, t462, t464)
}
