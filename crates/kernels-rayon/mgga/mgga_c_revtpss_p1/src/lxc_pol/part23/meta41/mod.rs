//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta41 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk296;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk297;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk298;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk299;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk300;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk301;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk302;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk303;
use chunk8::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk304;
use chunk9::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk305;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta41(t235: f64, t822: f64, t239: f64, t820: f64, t240: f64, t243: f64, t72: f64, t125: f64, t245: f64, t225: f64, t679: f64, t704: f64, t709: f64, t718: f64, t751: f64, t754: f64, t759: f64, t764: f64, t73: f64, t775: f64, t227: f64, t229: f64, t231: f64, t587: f64, t66: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t823 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk296(t235, t822);
        let t825 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk297(t239, t820, t823);
        let t826 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk298(t240, t243);
        let t827 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk299(t72, t826);
        let t828 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk300(t125, t245);
        let t830 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk301(t225, t679, t704, t709, t718, t751, t754, t759, t764);
        let t832 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk302(t243, t73);
        let (t833, t836) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk303(t775, t832, t227, t229, t830);
        let t837 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk304(t231, t836);
        let (t839, t843) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk305(t828, t837, t827, t587, t66);
    (t823, t825, t826, t827, t828, t830, t832, t833, t836, t837, t839, t843)
}
