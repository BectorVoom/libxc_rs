//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 944/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk944(t14648: f64, t2665: f64, t446: f64, t1212: f64, t2347: f64, t2349: f64, t10409: f64, t1882: f64, t4053: f64, t4129: f64, t668: f64, t505: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14649 = t2665 * t14648;
    let t14650 = t446 * t14649;
    let t14652 = t1212 * t2347;
    let t14653 = t14652 * t2349;
    let t14654 = t10409 * t14653;
    let t14655 = t446 * t14654;
    let t14657 = t1882 * t4053;
    let t14658 = t14657 / 27.0_f64;
    let t14659 = t4129 * t668;
    let t14660 = t14659 * t505;
    (t14650, t14653, t14655, t14657, t14658, t14660)
}
