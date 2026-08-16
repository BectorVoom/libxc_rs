//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta74 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk495;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk496;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk497;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk498;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk499;
use chunk5::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk500;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta74(t5: f64, t1406: f64, t1437: f64, t605: f64, t86: f64, t112: f64, t1408: f64, t95: f64, t50: f64, t103: f64, t100: f64, t104: f64, t92: f64, tau1: f64, t109: f64, t656: f64, t64: f64, t654: f64, t510: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1441 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk495(t5, t1406, t1437, t605, t86);
        let t1442 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk496(t112, t1441);
        let t1444 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk497(t1408);
        let (t1445, t1447, t1449, t1450, t1453) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk498(t1444, t95, t50, t103, t100, t104, t92, tau1);
        let (t1454, t1458) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk499(t109, t1453, t656, t64, t654);
        let t1459 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk500(t1458, t510);
    (t1441, t1442, t1444, t1445, t1447, t1449, t1450, t1453, t1454, t1458, t1459)
}
