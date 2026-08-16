//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta127 (260520-c91 hierarchical CSE).
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
mod chunk10;
mod chunk11;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk845;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk846;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk847;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk848;
use chunk4::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk849;
use chunk5::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk850;
use chunk6::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk851;
use chunk7::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk852;
use chunk8::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk853;
use chunk9::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk854;
use chunk10::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk855;
use chunk11::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk856;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta127(t3042: f64, t341: f64, t988: f64, t993: f64, t378: f64, t989: f64, t340: f64, t992: f64, t338: f64, t999: f64, t996: f64, t1071: f64, t994: f64, t1096: f64, t1079: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t3043 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk845(t3042, t341);
        let t3046 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk846(t988, t993);
        let t3047 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk847(t3046, t378);
        let t3052 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk848(t378, t989);
        let t3056 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk849(t340, t992);
        let t3057 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk850(t3056, t338);
        let t3058 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk851(t3057, t378);
        let t3059 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk852(t999);
        let t3060 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk853(t3059, t996);
        let t3063 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk854(t1071, t994);
        let t3066 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk855(t1096, t999);
        let t3067 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk856(t1079, t3066);
    (t3043, t3046, t3047, t3052, t3056, t3057, t3058, t3059, t3060, t3063, t3066, t3067)
}
