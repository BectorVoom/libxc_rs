//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 723/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk723(t79: f64, t11126: f64, t11223: f64, t11330: f64, t11389: f64, t370: f64, t27: f64, t89: f64, t1904: f64, t2992: f64, t1564: f64, t446: f64, t11174: f64, t17: f64) -> (f64, f64, f64, f64, f64) {
    let t80 = 0.1e-59_f64 < t79;
    let t11392 = piecewise3(t80, t11126 + t11223 + t11330 + t11389, 0.0_f64);
    let t11393 = t370 * t11392;
    let t11395 = t89 * t27 * t11393;
    let t11397 = t2992 * t1904;
    let t11398 = t1564 * t11397;
    let t11399 = t446 * t11398;
    let t11401 = t11174 * t17;
    (t11392, t11395, t11397, t11399, t11401)
}
