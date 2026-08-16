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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk680;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk681;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk682;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk683;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk684;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk685;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk686;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk687;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta97(t890: f64, t261: f64, t45: f64, t57: f64, t190: f64, t2258: f64, t706: f64, t2251: f64, t766: f64, t80: f64, t770: f64, t83: f64, zeta_threshold: f64, t125: f64, t215: f64, t123: f64, t781: f64, t124: f64, t68: f64, t138: f64, t251: f64, t785: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t2408 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk680(t890);
        let (t2410, t2411) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk681(t261);
        let (t2414, t2416, t2430) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk682(t45, t57, t190, t2258, t706, t2251, t766, t80, t770, t83, zeta_threshold);
        let t2434 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk683(t125, t215);
        let t2435 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk684(t123, t2434);
        let (t2437, t2438) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk685(t2435, t781, t124, t68);
        let t2439 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk686(t138, t2438);
        let t2440 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk687(t251, t785);
    (t2408, t2410, t2411, t2414, t2416, t2430, t2434, t2435, t2437, t2438, t2439, t2440)
}
