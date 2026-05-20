//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta32 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk220;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk221;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk222;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk223;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk224;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk225;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk226;
use chunk7::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk227;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta32<F: Float>(t625: F, t44: F, t49: F, t56: F, t614: F, t617: F, t620: F, t38: F, t45: F, t78: F, t57: F, t81: F, t606: F, t77: F, t608: F, t71: F, t85: F, t5: F, t599: F, t603: F, t91: F, t117: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t626, t627, t628, t631) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk220::<F>(t625, t44, t49, t56, t614, t617, t620, t38, t45);
        let t633 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk221::<F>(t631, t78);
        let t635 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk222::<F>(t57);
        let t637 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk223::<F>(t635, t81);
        let t640 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk224::<F>(t606, t633, t637);
        let (t641, t644) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk225::<F>(t640, t77, t608, t628, t71, t85);
        let t648 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk226::<F>(t5, t599, t603, t644, t91);
        let t649 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk227::<F>(t117, t648);
    (t626, t627, t628, t631, t633, t635, t637, t640, t641, t644, t648, t649)
}
