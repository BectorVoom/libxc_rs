//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta19 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk148;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk149;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk150;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk151;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk152;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk153;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk154;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk155;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta19(t360: f64, sigma0: f64, t34: f64, t35: f64, rho0: f64, t354: f64, t335: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t361, t362, t363) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk148(t360, sigma0);
        let t364 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk149(t362, t363);
        let t365 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk150(t34);
        let (t366, t368) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk151(t365, t35, rho0);
        let t369 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk152(t364, t368);
        let t370 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk153(t354, t369);
        let t371 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk154(t335);
        let t372 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk155(t371);
    (t361, t362, t363, t364, t365, t366, t368, t369, t370, t371, t372)
}
