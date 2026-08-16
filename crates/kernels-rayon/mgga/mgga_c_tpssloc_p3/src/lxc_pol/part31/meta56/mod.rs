//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta56 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk367;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk368;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk369;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk370;
use chunk4::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk371;
use chunk5::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk372;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta56(t25: f64, t265: f64, t394: f64, t1068: f64, t1070: f64, t193: f64, t336: f64, t873: f64, t890: f64, t916: f64, t956: f64, t958: f64, t963: f64, t396: f64, t40: f64, t606: f64, t607: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t268: f64, t405: f64, t878: f64, t154: f64, t486: f64, t636: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1074, t1079) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk367(t25, t265, t394, t1068, t1070, t193, t336, t873, t890, t916, t956, t958, t963, t396, t40, t606, t607, dens_threshold, rho0, zeta_threshold);
        let t1081 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk368(t606);
        let t1086 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk369(t268, t405, t878);
        let (t1087, t1088) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk370(t1086, t154, t486);
        let t1089 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk371(t636);
        let t1090 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk372(t1089, t607);
    (t1074, t1079, t1081, t1086, t1087, t1088, t1089, t1090)
}
