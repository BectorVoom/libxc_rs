//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta263 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1181;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1182;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1183;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1184;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1185;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1186;
use chunk6::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1187;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta263(t25: f64, t265: f64, t394: f64, t2165: f64, t671: f64, t6834: f64, t2116: f64, t40: f64, t607: f64, t6678: f64, t1170: f64, t2123: f64, t2121: f64, t2127: f64, t6686: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t1176: f64, t461: f64, t491: f64, t225: f64, t497: f64, t1090: f64, t1186: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7271, t7274, t7279, t7280, t7282, t7283) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1181(t25, t265, t394, t2165, t671, t6834, t2116, t40, t607, t6678, t1170, t2123, t2121, t2127, t6686, dens_threshold, rho0, zeta_threshold);
        let t7284 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1182(t1176, t461);
        let t7285 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1183(t491, t7284);
        let t7286 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1184(t225, t497);
        let t7287 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1185(t1090, t7286);
        let t7288 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1186(t7285, t7287);
        let t7291 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1187(t1186, t2123);
    (t7271, t7274, t7279, t7280, t7282, t7283, t7284, t7285, t7286, t7287, t7288, t7291)
}
