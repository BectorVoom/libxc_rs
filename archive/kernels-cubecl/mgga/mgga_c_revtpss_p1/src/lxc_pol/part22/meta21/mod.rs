//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta21 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk164;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk165;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk166;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk167;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk168;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk169;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta21<F: Float>(t30: F, t33: F, t265: F, t395: F, t45: F, t57: F, dens_threshold: F, rho0: F, rho1: F, zeta_threshold: F, t268: F, t269: F, t281: F, t282: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t398, t403, t404) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk164::<F>(t30, t33, t265, t395, t45, t57, dens_threshold, rho0, rho1, zeta_threshold);
        let t406 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk165::<F>(t268, t269, t404);
        let t408 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk166::<F>(t406);
        let t409 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk167::<F>(t406);
        let (t412, t414) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk168::<F>(t406, t404);
        let (t416, t418, t421, t422) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk169::<F>(t281, t282, t414, t406, t409, t412);
    (t398, t403, t404, t406, t408, t409, t412, t414, t416, t418, t421, t422)
}
