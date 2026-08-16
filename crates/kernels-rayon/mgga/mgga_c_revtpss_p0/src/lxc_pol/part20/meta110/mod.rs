//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta110 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk631;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk632;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk633;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk634;
use chunk4::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk635;
use chunk5::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk636;
use chunk6::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk637;
use chunk7::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk638;
use chunk8::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk639;
use chunk9::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk640;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta110(t357: f64, t905: f64, t606: f64, t3093: f64, t3092: f64, t1066: f64, t2858: f64, t247: f64, t1052: f64, t369: f64, t361: f64, t351: f64, t1065: f64, t126: f64, t906: f64, t1063: f64, t1086: f64, t994: f64, t3090: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3094, t3095) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk631(t357, t905, t606);
        let t3096 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk632(t3093, t3095);
        let t3097 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk633(t3092, t3096);
        let t3101 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk634(t1066, t2858, t247);
        let t3105 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk635(t1052, t369, t361);
        let t3106 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk636(t3105, t351);
        let t3109 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk637(t1065, t126);
        let t3111 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk638(t3109, t906, t247);
        let (t3112, t3114) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk639(t1063, t3111, t1086, t994);
        let t3115 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk640(t3090, t3114);
    (t3094, t3095, t3096, t3097, t3101, t3105, t3106, t3109, t3111, t3112, t3114, t3115)
}
