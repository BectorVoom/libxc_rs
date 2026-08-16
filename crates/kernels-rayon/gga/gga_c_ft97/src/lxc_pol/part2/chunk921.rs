//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 921/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk921(t1168: f64, t2526: f64, t2568: f64, t242: f64, t2459: f64, t729: f64, t762: f64, t10002: f64, t3864: f64, t2574: f64, t3837: f64, t773: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14254 = t1168 * t2526;
    let t14255 = t2568 * t14254;
    let t14256 = t242 * t14255;
    let t14259 = t1168 * t2459;
    let t14261 = t729 * t762 * t14259;
    let t14264 = t10002 * t3864;
    let t14265 = t242 * t14264;
    let t14269 = t2574 * t773 * t3837;
    (t14255, t14256, t14261, t14264, t14265, t14269)
}
