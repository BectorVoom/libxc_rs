//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta74 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk495;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk496;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk497;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk498;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk499;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta74(t112: f64, t1441: f64, t1408: f64, t95: f64, t50: f64, t103: f64, t100: f64, t104: f64, t92: f64, tau1: f64, t109: f64, t656: f64, t64: f64, t654: f64, t510: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t1442 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk495(t112, t1441);
        let t1444 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk496(t1408);
        let (t1445, t1447, t1449, t1450, t1453) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk497(t1444, t95, t50, t103, t100, t104, t92, tau1);
        let (t1454, t1458) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk498(t109, t1453, t656, t64, t654);
        let t1459 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk499(t1458, t510);
    (t1442, t1444, t1445, t1447, t1449, t1450, t1453, t1454, t1458, t1459)
}
