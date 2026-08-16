//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta41 (260520-c91 hierarchical CSE).
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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk290;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk291;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk292;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk293;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk294;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk295;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk296;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk297;
use chunk8::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk298;
use chunk9::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk299;
use chunk10::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk300;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta41(t240: f64, t815: f64, t812: f64, t241: f64, t244: f64, t67: f64, t120: f64, t246: f64, t225: f64, t680: f64, t705: f64, t710: f64, t719: f64, t752: f64, t755: f64, t760: f64, t765: f64, t68: f64, t776: f64, t228: f64, t230: f64, t232: f64, t590: f64, t61: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t816 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk290(t240, t815);
        let t817 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk291(t812, t816);
        let t819 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk292(t241, t244, t67);
        let t820 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk293(t120, t246);
        let t822 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk294(t225, t680, t705, t710, t719, t752, t755, t760, t765);
        let t824 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk295(t244, t68);
        let (t825, t828) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk296(t776, t824, t228, t230, t822);
        let t829 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk297(t232, t828);
        let t831 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk298(t819, t820, t829);
        let t835 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk299(t590, t61);
        let t836 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk300(t241, t835);
    (t816, t817, t819, t820, t822, t824, t825, t828, t829, t831, t835, t836)
}
