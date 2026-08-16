//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1139/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1139(t39609: f64, t1285: f64, t9218: f64, t16: f64, t185: f64, t520: f64, t1284: f64, t17: f64, t9861: f64, t3719: f64) -> (f64, f64, f64, f64, f64) {
    let t39610 = 960.0_f64 * t39609;
    let t39611 = t9218 * t1285;
    let t39612 = 480.0_f64 * t39611;
    let t39615 = 24.0_f64 * t16 * t520 * t185;
    let t39620 = t17 * t1284 * t9861;
    let t39621 = 4.0_f64 * t39620;
    let t39622 = t3719 * t3719;
    (t39610, t39612, t39615, t39621, t39622)
}
