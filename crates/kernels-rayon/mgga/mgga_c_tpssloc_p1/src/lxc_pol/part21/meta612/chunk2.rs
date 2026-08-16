//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2382/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2382(t2225: f64, t3824: f64, t1287: f64, t9214: f64, t12129: f64, t588: f64, t39033: f64, t522: f64, t1285: f64, t9216: f64, t9218: f64, t16: f64, t185: f64, t520: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39595 = 120.0_f64 * t2225 * t3824;
    let t39596 = t9214 * t1287;
    let t39601 = t588 * t12129;
    let t39603 = t39033 * t522;
    let t39609 = t9216 * t1285;
    let t39611 = t9218 * t1285;
    let t39615 = 24.0_f64 * t16 * t520 * t185;
    (t39595, t39596, t39601, t39603, t39609, t39611, t39615)
}
