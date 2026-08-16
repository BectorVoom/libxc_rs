//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta41 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk300;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk301;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk302;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk303;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk304;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk305;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk306;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk307;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk308;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk309;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta41(t243: f64, t814: f64, t124: f64, t800: f64, t813: f64, t213: f64, t225: f64, t232: f64, t235: f64, t239: f64, t240: f64, t72: f64, t125: f64, t245: f64, t679: f64, t704: f64, t709: f64, t718: f64, t751: f64, t754: f64, t759: f64, t764: f64, t73: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t815, t816) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk300(t243, t814, t124, t800);
        let (t819, t820) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk301(t815, t816, t813, t213, t225);
        let (t821, t822) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk302(t232);
        let t823 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk303(t235, t822);
        let t825 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk304(t239, t820, t823);
        let t826 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk305(t240, t243);
        let t827 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk306(t72, t826);
        let t828 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk307(t125, t245);
        let t830 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk308(t225, t679, t704, t709, t718, t751, t754, t759, t764);
        let t832 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk309(t243, t73);
    (t816, t819, t820, t821, t822, t823, t825, t826, t827, t828, t830, t832)
}
