//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta34 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk251;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk252;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk253;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk254;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk255;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk256;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta34(t103: f64, t662: f64, t100: f64, t657: f64, t660: f64, t92: f64, t96: f64, t109: f64, t656: f64, t64: f64, t654: f64, t510: f64, t3: f64, t60: f64, t120: f64, t118: f64, t142: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t663, t666) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk251(t103, t662, t100, t657, t660, t92, t96);
        let (t667, t671) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk252(t109, t656, t666, t64, t654);
        let t672 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk253(t510, t671);
        let t676 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk254(t3, t60);
        let t677 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk255(t120, t676);
        let t680 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk256(t118, t142, t677);
    (t663, t666, t667, t671, t672, t676, t677, t680)
}
