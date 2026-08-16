//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta20 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk155;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk156;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk157;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk158;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk159;
use chunk5::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk160;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta20(t349: f64, t381: f64, t362: f64, t68: f64, t353: f64, t254: f64, t193: f64, t293: f64, t328: f64, t330: f64, t336: f64, t265: f64, t25: f64, t28: f64, t40: f64, t52: f64, dens_threshold: f64, rho0: f64, rho1: f64, zeta_threshold: f64, t268: f64, t269: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t382, t383) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk155(t349, t381, t362, t68);
        let t384 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk156(t381, t383);
        let (t386, t388) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk157(t353, t384, t254);
        let (t390, t396, t394) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk158(t382, t388, t193, t293, t328, t330, t336, t265);
        let (t399, t404, t405) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk159(t25, t28, t265, t396, t40, t52, dens_threshold, rho0, rho1, zeta_threshold);
        let t407 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk160(t268, t269, t405);
    (t382, t383, t384, t386, t388, t390, t396, t394, t399, t404, t405, t407)
}
