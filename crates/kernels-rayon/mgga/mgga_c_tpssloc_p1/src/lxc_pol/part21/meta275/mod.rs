//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta275 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1549;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1550;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1551;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta275(t153: f64, t9862: f64, t2371: f64, t2531: f64, t2528: f64, t2517: f64, t607: f64, t707: f64, t2652: f64, t2663: f64, t181: f64, t686: f64, t781: f64, t756: f64, t118: f64, t753: f64, t2375: f64, t677: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9863, t9864, t9866, t9868, t9869, t9871, t9874) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1549(t153, t9862, t2371, t2531, t2528, t2517, t607, t707, t2652, t2663, t181, t686, t781);
        let (t9876, t9879) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1550(t756, t9874, t118, t753);
        let (t9880, t9882) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1551(t2375, t9879, t2371, t677);
    (t9863, t9864, t9866, t9868, t9869, t9871, t9874, t9876, t9879, t9880, t9882)
}
