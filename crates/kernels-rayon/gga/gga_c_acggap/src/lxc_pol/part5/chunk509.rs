//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 509/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk509(t218: f64, t761: f64, t219: f64, t777: f64, t1072: f64, t2: f64, t39: f64, t575: f64, t661: f64, t660: f64, t203: f64, t328: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2666 = t761 * t218;
    let t2667 = t2666 * t219;
    let t2668 = t777 * t2667;
    let t2669 = 6.0_f64 * t2668;
    let t2670 = t1072 * t2;
    let t2671 = t2670 * t39;
    let t2673 = t661 * t575;
    let t2674 = t660 * t2673;
    let t2676 = t203 * t328;
    (t2666, t2667, t2668, t2669, t2670, t2671, t2673, t2674, t2676)
}
