//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta321 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1138;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1139;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1140;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta321(t2225: f64, t3824: f64, t1287: f64, t9214: f64, t12129: f64, t588: f64, t39033: f64, t522: f64, t39035: f64, t39031: f64, t1285: f64, t9216: f64, t9218: f64, t16: f64, t185: f64, t520: f64, t1284: f64, t17: f64, t9861: f64, t3719: f64, t12012: f64, t12303: f64, t193: f64, t3918: f64, t3919: f64, t3924: f64, t39590: f64, t39593: f64, t5126: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39595, t39597, t39602, t39604, t39606, t39608, t39609) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1138(t2225, t3824, t1287, t9214, t12129, t588, t39033, t522, t39035, t39031, t1285, t9216);
        let (t39610, t39612, t39615, t39621, t39622) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1139(t39609, t1285, t9218, t16, t185, t520, t1284, t17, t9861, t3719);
        let t39626 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1140(t12012, t12303, t193, t3918, t3919, t3924, t39590, t39593, t39595, t39597, t39602, t39604, t39606, t39608, t39610, t39612, t39615, t39621, t39622, t5126);
    (t39595, t39597, t39602, t39604, t39606, t39608, t39610, t39612, t39615, t39621, t39622, t39626)
}
