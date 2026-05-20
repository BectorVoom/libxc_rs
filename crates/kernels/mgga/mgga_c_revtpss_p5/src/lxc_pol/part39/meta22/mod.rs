//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta22 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk147;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk148;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk149;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk150;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta22<F: Float>(t281: F, t282: F, t414: F, t406: F, t409: F, t412: F, t408: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t416, t418, t421, t422) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk147::<F>(t281, t282, t414, t406, t409, t412);
        let (t424, t426) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk148::<F>(t408, t422, t406);
        let (t431, t434, t435) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk149::<F>(t406, t409, t412, t416);
        let t439 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk150::<F>(t406);
    (t416, t418, t421, t422, t424, t426, t431, t434, t435, t439)
}
