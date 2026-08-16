//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1379/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1379(t123: f64, t553: f64, t27441: f64, t1900: f64, t438: f64, t1122: f64, t3105: f64, t8446: f64, t3103: f64, t3120: f64, t3236: f64, t8415: f64) -> (f64, f64, f64, f64, f64) {
    let t27442 = t553 * t123;
    let t27443 = t27441 * t27442;
    let t27448 = t438 * t1900;
    let t27449 = t553 * t1122 * t27448;
    let t27453 = t8446 * t3105;
    let t27455 = t3103 * t27453 * t3120;
    let t27461 = t8415 * t3236;
    (t27443, t27449, t27453, t27455, t27461)
}
