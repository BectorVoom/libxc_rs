//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta91 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk648;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk649;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk650;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk651;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk652;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk653;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk654;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk655;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk656;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta91(t2236: f64, t25: f64, t2219: f64, t2221: f64, t2223: f64, t2226: f64, t2228: f64, t2230: f64, t2233: f64, t2235: f64, t599: f64, t602: f64, t89: f64, t90: f64, t29: f64, t644: f64, t606: f64, t70: f64, t2: f64, t580: f64, t17: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t2237 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk648(t2236);
        let (t2239, t2240, t2242) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk649(t2237, t25, t2219, t2221, t2223, t2226, t2228, t2230, t2233, t2235, t599, t602);
        let t2246 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk650(t89, t90);
        let t2247 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk651(t2246, t29);
        let t2248 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk652(t644);
        let t2251 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk653(t606);
        let (t2252, t2255) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk654(t2251, t70, t2, t580);
        let t2256 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk655(t17, t2255);
        let t2257 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk656(t2256);
    (t2237, t2239, t2240, t2242, t2246, t2247, t2248, t2251, t2252, t2255, t2256, t2257)
}
