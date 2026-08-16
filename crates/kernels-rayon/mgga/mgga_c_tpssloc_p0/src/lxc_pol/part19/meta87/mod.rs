//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta87 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk498;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk499;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk500;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk501;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk502;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk503;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk504;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta87(t2617: f64, t816: f64, t809: f64, t838: f64, t842: f64, t233: f64, t813: f64, t236: f64, t240: f64, t812: f64, t828: f64, t232: f64, t819: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2618, t2621, t2623) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk498(t2617, t816, t809, t838, t842);
        let t2627 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk499(t233, t813);
        let t2628 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk500(t236, t2627);
        let (t2629, t2630, t2631) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk501(t240, t2628, t812, t828);
        let t2632 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk502(t232);
        let t2633 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk503(t2631, t2632);
        let t2635 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk504(t2633, t819, t820);
    (t2618, t2621, t2623, t2627, t2628, t2629, t2630, t2631, t2632, t2633, t2635)
}
