//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 931/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk931(t2938: f64, t4370: f64, t898: f64, t904: f64, t231: f64, t893: f64, t1270: f64, t8640: f64, t2253: f64, t4372: f64, t668: f64, t505: f64) -> (f64, f64, f64, f64, f64) {
    let t14437 = t2938 * t4370;
    let t14439 = t898 * t14437 * t904;
    let t14442 = t231 * t893;
    let t14445 = t8640 * t1270;
    let t14448 = 2.0_f64 / 3.0_f64 * t2253 * t4372;
    let t14449 = t4370 * t668;
    let t14450 = t14449 * t505;
    (t14439, t14442, t14445, t14448, t14450)
}
