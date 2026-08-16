//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta25 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk176;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk177;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk178;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk179;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk180;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk181;
use chunk6::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk182;
use chunk7::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk183;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta25<F: Float>(t461: F, t464: F, t481: F, t484: F, t225: F, t473: F, t460: F, t198: F, t336: F, t424: F, t452: F, t454: F, t265: F, t33: F, t57: F, t398: F, dens_threshold: F, rho1: F, zeta_threshold: F, t117: F, t93: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t487 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk176::<F>(t461, t464, t481, t484);
        let (t488, t489) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk177::<F>(t225, t487, t473);
        let t490 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk178::<F>(t487, t489);
        let (t493, t494) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk179::<F>(t460, t490);
        let t495 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk180::<F>(t488, t494);
        let (t498, t504, t502) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk181::<F>(t460, t495, t198, t336, t424, t452, t454, t265);
        let t508 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk182::<F>(t33, t265, t504, t57, t398, dens_threshold, rho1, zeta_threshold);
        let t511 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk183::<F>(t117, t93);
    (t487, t488, t489, t490, t493, t494, t495, t498, t504, t502, t508, t511)
}
