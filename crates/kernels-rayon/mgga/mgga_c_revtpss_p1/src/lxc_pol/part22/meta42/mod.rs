//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta42 (260520-c91 hierarchical CSE).
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
mod chunk9;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk310;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk311;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk312;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk313;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk314;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk315;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk316;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk317;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk318;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk319;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta42(t775: f64, t832: f64, t227: f64, t229: f64, t830: f64, t231: f64, t828: f64, t827: f64, t587: f64, t66: f64, t240: f64, t243: f64, t247: f64, t237: f64, t233: f64, t235: f64, t239: f64, t820: f64, t205: f64, t242: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t833 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk310(t775, t832);
        let t836 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk311(t227, t229, t830, t833);
        let t837 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk312(t231, t836);
        let (t839, t843) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk313(t828, t837, t827, t587, t66);
        let t844 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk314(t240, t843);
        let (t848, t849) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk315(t243, t844, t247, t237, t233, t235);
        let t851 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk316(t239, t820, t849);
        let t853 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk317(t205, t242);
        let t854 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk318(t240, t853);
        let t855 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk319(t72, t854);
    (t833, t836, t837, t839, t843, t844, t848, t849, t851, t853, t854, t855)
}
