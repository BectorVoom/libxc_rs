//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta309 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1559;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1560;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1561;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta309(t11129: f64, t11292: f64, t3403: f64, t1164: f64, t1143: f64, t3375: f64, t1156: f64, t1124: f64, t3331: f64, t1136: f64, t3333: f64, t1137: f64, t11282: f64, t440: f64, t11285: f64, t11135: f64, t11203: f64, t11161: f64, t11170: f64, t11197: f64, t11200: f64, t11206: f64, t11209: f64, t11211: f64, t11213: f64, t11215: f64, t11217: f64, t11221: f64, t11224: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11294, t11296, t11297, t11300, t11303, t11306) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1559(t11129, t11292, t3403, t1164, t1143, t3375, t1156, t1124, t3331, t1136, t3333);
        let (t11307, t11310) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1560(t11306, t1137, t11282, t440);
        let (t11311, t11314, t11317, t11328) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1561(t11129, t11285, t11135, t11203, t11161, t11170, t11197, t11200, t11206, t11209, t11211, t11213, t11215, t11217, t11221, t11224);
    (t11294, t11296, t11297, t11300, t11303, t11306, t11307, t11310, t11311, t11314, t11317, t11328)
}
