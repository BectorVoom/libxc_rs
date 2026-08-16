//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 938/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk938(t8656: f64, t8675: f64, t8682: f64, t1736: f64, t639: f64, t2281: f64, t422: f64, t71: f64, t8618: f64, t2284: f64, t8640: f64, t2007: f64, t37627: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39483 = t8675 * t8656;
    let t39485 = t8675 * t8682;
    let t39487 = t1736 * t639;
    let t39495 = t422 * t2281;
    let t39514 = t71 * t8618;
    let t39524 = t8640 * t2284;
    let t39533 = t2007 * t37627;
    (t39483, t39485, t39487, t39495, t39514, t39524, t39533)
}
