//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 555/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk555(t252: f64, t2591: f64, t798: f64, t852: f64, t225: f64, t799: f64, t154: f64, t2559: f64, t222: f64, t2563: f64, t805: f64, t119: f64, t2379: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2592 = t2591 * t252;
    let t2594 = t798 * t852;
    let t2597 = t799 * t225;
    let t2600 = t2559 * t154;
    let t2602 = 35.0_f64 / 432.0_f64 * t2600 * t222;
    let t2603 = t2563 * t805;
    let t2605 = t119 * t2379;
    (t2592, t2594, t2597, t2600, t2602, t2603, t2605)
}
