//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta22 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk162;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk163;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk164;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk165;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk166;
use chunk5::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk167;
use chunk6::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk168;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta22(t382: f64, t388: f64, t193: f64, t293: f64, t328: f64, t330: f64, t336: f64, t265: f64, t25: f64, t28: f64, t40: f64, t52: f64, dens_threshold: f64, rho0: f64, rho1: f64, zeta_threshold: f64, t268: f64, t269: f64, t281: f64, t282: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t390, t396, t394) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk162(t382, t388, t193, t293, t328, t330, t336, t265);
        let (t399, t404, t405) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk163(t25, t28, t265, t396, t40, t52, dens_threshold, rho0, rho1, zeta_threshold);
        let t407 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk164(t268, t269, t405);
        let t409 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk165(t407);
        let t410 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk166(t407);
        let (t413, t415) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk167(t407, t405);
        let t417 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk168(t281, t282, t415);
    (t390, t396, t394, t399, t404, t405, t407, t409, t410, t413, t415, t417)
}
