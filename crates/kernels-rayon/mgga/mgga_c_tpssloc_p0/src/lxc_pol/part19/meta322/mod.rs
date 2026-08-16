//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta322 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1141;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1142;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta322(t2225: f64, t3696: f64, t12124: f64, t588: f64, t592: f64, t1287: f64, t9212: f64, t1285: f64, t12083: f64, t17: f64, t750: f64, t2516: f64, t3681: f64, t12126: f64, t3914: f64, t9218: f64, t118: f64, t142: f64, t39283: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39629, t39631, t39633, t39635, t39637, t39640, t39642) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1141(t2225, t3696, t12124, t588, t592, t1287, t9212, t1285, t12083, t17, t750, t2516, t3681);
        let (t39643, t39645, t39649, t39655, t39658) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1142(t39642, t12126, t592, t3914, t1287, t9218, t118, t142, t39283);
    (t39629, t39631, t39633, t39635, t39637, t39640, t39643, t39645, t39649, t39655, t39658)
}
