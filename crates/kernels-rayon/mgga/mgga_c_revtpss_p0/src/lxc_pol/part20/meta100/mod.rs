//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta100 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk576;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk577;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk578;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk579;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk580;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk581;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk582;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk583;
use chunk8::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk584;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta100(t631: f64, t2251: f64, t2850: f64, t128: f64, t2297: f64, t904: f64, t2258: f64, t905: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t2851 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk576(t631);
        let t2852 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk577(t2851);
        let t2853 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk578(t2251, t2852);
        let (t2854, t2855) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk579(t2850, t2853, t128);
        let t2857 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk580(t2297);
        let t2858 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk581(t2251, t2857);
        let (t2859, t2860) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk582(t2858, t904, t128);
        let t2862 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk583(t2258, t905);
        let (t2863, t2864) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk584(t2862, t904, t128);
    (t2851, t2852, t2853, t2854, t2855, t2857, t2858, t2859, t2860, t2862, t2863, t2864)
}
