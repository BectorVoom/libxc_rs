//! MGGA_C_REVTPSS lxc pol kernel — _part30_v4rho3sigma_5 meta269 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1181;
use chunk1::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1182;
use chunk2::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1183;
use chunk3::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1184;
use chunk4::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1185;
use chunk5::mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1186;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_meta269<F: Float>(t670: F, t7330: F, t572: F, t117: F, t7002: F, t2121: F, t38: F, t2247: F, t55: F, t60: F, t606: F, t6971: F, t72: F, t1927: F, t2122: F, t6977: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t7331, t7333, t7334, t7336, t7565) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1181::<F>(t670, t7330, t572, t117, t7002, t2121, t38);
        let t7566 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1182::<F>(t2247, t7565);
        let t7571 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1183::<F>(t55, t60);
        let (t7574, t7575) = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1184::<F>(t606, t6971, t7571, t72);
        let t7576 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1185::<F>(t1927, t7575);
        let t7579 = mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1186::<F>(t2122, t6977);
    (t7331, t7333, t7334, t7336, t7565, t7566, t7571, t7574, t7575, t7576, t7579)
}
