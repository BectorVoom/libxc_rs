//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta18 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk142;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk143;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk144;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk145;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk146;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk147;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta18(t340: f64, t60: f64, t285: f64, t221: f64, t339: f64, t225: f64, t68: f64, t336: f64, t293: f64, t328: f64, t330: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t341, t343) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk142(t340, t60, t285);
        let t344 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk143(t343);
        let (t346, t349) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk144(t341, t344, t221, t339);
        let (t350, t353) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk145(t221, t341, t225, t349);
        let t354 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk146(t353, t68);
        let (t357, t358, t360) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk147(t336, t68, t225, t293, t328, t330);
    (t341, t343, t344, t346, t349, t350, t353, t354, t357, t358, t360)
}
