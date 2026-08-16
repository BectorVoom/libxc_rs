//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta93 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk642;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk643;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk644;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk645;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk646;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta93(t662: f64, t2349: f64, t2248: f64, t103: f64, t100: f64, t2336: f64, t2343: f64, t2346: f64, t657: f64, t660: f64, t92: f64, t96: f64, t109: f64, t656: f64, t2327: f64, t2328: f64, t2333: f64, t64: f64, t510: f64, t177: f64, t738: f64, t745: f64, t746: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2350, t2351, t2354, t2355, t2358) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk642(t662, t2349, t2248, t103, t100, t2336, t2343, t2346, t657, t660, t92, t96);
        let (t2359, t2363) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk643(t109, t2358, t656, t2327, t2328, t2333, t64);
        let (t2364, t2367, t2368) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk644(t2363, t510, t177, t738);
        let t2369 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk645(t745);
        let t2371 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk646(t2368, t2369, t746);
    (t2350, t2351, t2354, t2355, t2358, t2359, t2363, t2364, t2367, t2368, t2369, t2371)
}
