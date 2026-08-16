//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta40 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk277;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk278;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk279;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk280;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk281;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk282;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk283;
use chunk7::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk284;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta40(t233: f64, t236: f64, t240: f64, t812: f64, t241: f64, t244: f64, t67: f64, t120: f64, t246: f64, t225: f64, t680: f64, t705: f64, t710: f64, t719: f64, t752: f64, t755: f64, t760: f64, t765: f64, t68: f64, t776: f64, t228: f64, t230: f64, t232: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t813, t814) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk277(t233);
        let t815 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk278(t236, t814);
        let (t816, t817) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk279(t240, t815, t812);
        let t819 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk280(t241, t244, t67);
        let t820 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk281(t120, t246);
        let t822 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk282(t225, t680, t705, t710, t719, t752, t755, t760, t765);
        let (t824, t825, t828) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk283(t244, t68, t776, t228, t230, t822);
        let t829 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk284(t232, t828);
    (t813, t814, t815, t816, t817, t819, t820, t822, t824, t825, t828, t829)
}
