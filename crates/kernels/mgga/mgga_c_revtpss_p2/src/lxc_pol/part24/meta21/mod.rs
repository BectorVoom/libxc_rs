//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta21 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk164;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk165;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk166;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk167;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk168;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta21<F: Float>(t406: F, t404: F, t281: F, t282: F, t409: F, t408: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t412, t414) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk164::<F>(t406, t404);
        let (t416, t418, t421, t422) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk165::<F>(t281, t282, t414, t406, t409, t412);
        let (t424, t426) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk166::<F>(t408, t422, t406);
        let (t431, t434, t435) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk167::<F>(t406, t409, t412, t416);
        let t439 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk168::<F>(t406);
    (t412, t414, t416, t418, t421, t422, t424, t426, t431, t434, t435, t439)
}
