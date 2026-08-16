//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta655 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2422;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2423;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta655(t10828: f64, t300: f64, t4475: f64, t49514: f64, t10753: f64, t4488: f64, t959: f64, t14480: f64, t2940: f64, t2930: f64, t1581: f64, t13716: f64, t2904: f64, t952: f64, t10623: f64, t4498: f64, t4493: f64, t10629: f64, t14259: f64, t4471: f64, t14260: f64, t13663: f64, t13718: f64, t49082: f64, t49084: f64, t49086: f64, t49088: f64, t49090: f64, t49092: f64, t49095: f64, t49228: f64, t49244: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t49535, t49538, t49540, t49544, t49548) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2422(t10828, t300, t4475, t49514, t10753, t4488, t959, t14480, t2940, t2930, t1581, t13716, t2904, t952);
        let (t49550, t49552, t49556, t49558, t49560, t49562, t49563) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2423(t10623, t4498, t4493, t10629, t14259, t4471, t959, t14260, t2940, t13663, t13718, t49082, t49084, t49086, t49088, t49090, t49092, t49095, t49228, t49244, t49535, t49538, t49540, t49544, t49548);
    (t49535, t49538, t49540, t49544, t49548, t49550, t49552, t49556, t49558, t49560, t49562, t49563)
}
