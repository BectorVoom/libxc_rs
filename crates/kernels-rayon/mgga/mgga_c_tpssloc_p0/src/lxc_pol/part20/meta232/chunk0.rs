//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1324/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1324(t789: f64, t9541: f64, t2563: f64, t2582: f64, t2566: f64, t786: f64, t2578: f64, t2570: f64, t792: f64, t118: f64, t2379: f64, t794: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9542 = t9541 * t789;
    let t9544 = t2563 * t2582;
    let t9546 = t2566 * t786;
    let t9547 = t9546 * t2578;
    let t9549 = t792 * t2570;
    let t9551 = t118 * t794 * t2379;
    (t9542, t9544, t9546, t9547, t9549, t9551)
}
