//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta72 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk527;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk528;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk529;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk530;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk531;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta72(t103: f64, t1449: f64, t100: f64, t104: f64, t1445: f64, t1447: f64, t92: f64, t109: f64, t656: f64, t64: f64, t654: f64, t510: f64, t1409: f64, t185: f64, t40: f64, t52: f64, t707: f64, t73: f64, t76: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t1450, t1453) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk527(t103, t1449, t100, t104, t1445, t1447, t92);
        let (t1454, t1458) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk528(t109, t1453, t656, t64, t654);
        let t1459 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk529(t1458, t510);
        let t1462 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk530(t1409, t185);
        let (t1464, t1471) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk531(t40, t52, t1462, t707, t1409, t73, t76, zeta_threshold);
    (t1450, t1453, t1454, t1458, t1459, t1462, t1464, t1471)
}
