//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta328 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1611;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1612;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1613;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta328(t11720: f64, t3508: f64, t1214: f64, t248: f64, t11708: f64, t3514: f64, t11717: f64, t1210: f64, t11713: f64, t475: f64, t3509: f64, t3570: f64, t3506: f64, t11159: f64, t3440: f64, t11168: f64, t1177: f64, t135: f64, t3561: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11729, t11731, t11734) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1611(t11720, t3508, t1214, t248, t11708, t3514);
        let (t11737, t11738) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1612(t11717, t1210, t11713);
        let (t11739, t11741, t11745, t11746, t11748, t11751, t11754) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1613(t11720, t475, t1214, t248, t3509, t3570, t3506, t11159, t3440, t11168, t1177, t135, t3561);
    (t11729, t11731, t11734, t11737, t11738, t11739, t11741, t11745, t11746, t11748, t11751, t11754)
}
