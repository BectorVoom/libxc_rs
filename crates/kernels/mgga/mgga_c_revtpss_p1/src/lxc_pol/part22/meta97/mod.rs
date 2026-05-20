//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta97 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk680;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk681;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk682;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk683;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk684;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk685;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk686;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk687;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta97<F: Float>(t890: F, t261: F, t45: F, t57: F, t190: F, t2258: F, t706: F, t2251: F, t766: F, t80: F, t770: F, t83: F, zeta_threshold: F, t125: F, t215: F, t123: F, t781: F, t124: F, t68: F, t138: F, t251: F, t785: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t2408 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk680::<F>(t890);
        let (t2410, t2411) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk681::<F>(t261);
        let (t2414, t2416, t2430) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk682::<F>(t45, t57, t190, t2258, t706, t2251, t766, t80, t770, t83, zeta_threshold);
        let t2434 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk683::<F>(t125, t215);
        let t2435 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk684::<F>(t123, t2434);
        let (t2437, t2438) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk685::<F>(t2435, t781, t124, t68);
        let t2439 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk686::<F>(t138, t2438);
        let t2440 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk687::<F>(t251, t785);
    (t2408, t2410, t2411, t2414, t2416, t2430, t2434, t2435, t2437, t2438, t2439, t2440)
}
