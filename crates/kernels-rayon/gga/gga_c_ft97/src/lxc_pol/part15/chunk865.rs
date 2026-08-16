//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 865/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk865(t70: f64, t8639: f64, t41: f64, t1736: f64, t639: f64, t2281: f64, t422: f64, t71: f64, t8618: f64, t118: f64, t37993: f64, t38062: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t39447 = t8639 * t70;
    let t39448 = t41 * t39447;
    let t39487 = t1736 * t639;
    let t39495 = t422 * t2281;
    let t39514 = t71 * t8618;
    let t39538 = 1.0_f64 / t118 / t37993;
    let t39546 = 0.14978012345679012345e1_f64 * t38062;
    (t39448, t39487, t39495, t39514, t39538, t39546)
}
