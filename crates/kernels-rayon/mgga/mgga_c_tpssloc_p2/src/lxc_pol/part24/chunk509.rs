//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 509/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk509(t67: f64, t753: f64, t758: f64, t185: f64, t2250: f64, t707: f64, t152: f64, t32: f64, t2244: f64, t181: f64, t204: f64, t686: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2652 = t753 * t67;
    let t2653 = t2652 * t758;
    let t2654 = 0.36622894612013090108e-3_f64 * t2653;
    let t2655 = t185 * t2250;
    let t2657 = 4.0_f64 * t707 * t2655;
    let t2658 = t32 * t152;
    let t2659 = t185 * t2244;
    let t2661 = 12.0_f64 * t2658 * t2659;
    let t2663 = t686 * t204 * t181;
    (t2652, t2654, t2655, t2657, t2658, t2659, t2661, t2663)
}
