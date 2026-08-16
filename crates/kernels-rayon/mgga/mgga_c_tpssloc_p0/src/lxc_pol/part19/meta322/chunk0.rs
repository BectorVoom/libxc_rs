//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1141/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1141(t2225: f64, t3696: f64, t12124: f64, t588: f64, t592: f64, t1287: f64, t9212: f64, t1285: f64, t12083: f64, t17: f64, t750: f64, t2516: f64, t3681: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39628 = t2225 * t3696;
    let t39629 = 120.0_f64 * t39628;
    let t39630 = t588 * t12124;
    let t39631 = 16.0_f64 * t39630;
    let t39632 = t592 * t12124;
    let t39633 = 16.0_f64 * t39632;
    let t39634 = t9212 * t1287;
    let t39635 = 96.0_f64 * t39634;
    let t39636 = t9212 * t1285;
    let t39637 = 96.0_f64 * t39636;
    let t39639 = t17 * t12083 * t750;
    let t39640 = 4.0_f64 * t39639;
    let t39642 = t17 * t3681 * t2516;
    (t39629, t39631, t39633, t39635, t39637, t39640, t39642)
}
