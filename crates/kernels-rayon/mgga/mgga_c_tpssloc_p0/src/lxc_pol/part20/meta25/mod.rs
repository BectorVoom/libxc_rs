//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta25 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk188;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk189;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk190;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk191;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk192;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk193;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk194;
use chunk7::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk195;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta25(t456: f64, t467: f64, t485: f64, t488: f64, t466: f64, t477: f64, t68: f64, t470: f64, t254: f64, t193: f64, t336: f64, t425: f64, t453: f64, t455: f64, t265: f64, t28: f64, t52: f64, t399: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64, t112: f64, t88: f64, t25: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t491 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk188(t456, t467, t485, t488);
        let (t492, t493) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk189(t466, t491, t477, t68);
        let t494 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk190(t491, t493);
        let (t496, t498) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk191(t470, t494, t254);
        let (t500, t506, t504) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk192(t492, t498, t193, t336, t425, t453, t455, t265);
        let t510 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk193(t28, t265, t506, t52, t399, dens_threshold, rho1, zeta_threshold);
        let t513 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk194(t112, t88);
        let t514 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk195(t25);
    (t491, t492, t493, t494, t496, t498, t500, t506, t504, t510, t513, t514)
}
