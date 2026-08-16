//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta311 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1566;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1567;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1568;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta311(t11129: f64, t3403: f64, t11135: f64, t11203: f64, t11161: f64, t11170: f64, t11197: f64, t11200: f64, t11206: f64, t11209: f64, t11211: f64, t11213: f64, t11215: f64, t11217: f64, t11221: f64, t11224: f64, t11137: f64, t11139: f64, t11141: f64, t11143: f64, t11150: f64, t11156: f64, t11165: f64, t11174: f64, t11230: f64, t11233: f64, t11245: f64, t11259: f64, t11261: f64, t11266: f64, t1156: f64, t1119: f64, t3307: f64, t3264: f64, t1117: f64, t3315: f64, t3313: f64, t1128: f64, t3324: f64, t1124: f64, t3356: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11366, t11369, t11372, t11383) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1566(t11129, t3403, t11135, t11203, t11161, t11170, t11197, t11200, t11206, t11209, t11211, t11213, t11215, t11217, t11221, t11224);
        let t11398 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1567(t11137, t11139, t11141, t11143, t11150, t11156, t11165, t11174, t11230, t11233, t11245, t11259, t11261, t11266);
        let (t11399, t11400, t11403, t11405, t11407, t11409, t11410, t11415) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1568(t11383, t11398, t1156, t1119, t3307, t3264, t1117, t3315, t3313, t1128, t3324, t1124, t3356);
    (t11366, t11369, t11372, t11399, t11400, t11403, t11405, t11407, t11409, t11410, t11415)
}
