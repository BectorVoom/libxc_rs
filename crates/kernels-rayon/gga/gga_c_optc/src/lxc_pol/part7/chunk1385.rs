//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1385/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1385(t8905: f64, t8974: f64, t4356: f64, t1162: f64, t2367: f64, t8936: f64, t27481: f64, t9169: f64, t9171: f64, t1107: f64, t8914: f64, t9122: f64, t9124: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27575 = t8974 * t8905;
    let t27579 = t4356 * t8905;
    let t27587 = t1162 * t2367 * t8936;
    let t27590 = t9169 * t27481 * t9171;
    let t27592 = t1107 * t8914;
    let t27594 = t9122 * t27592 * t9124;
    (t27575, t27579, t27587, t27590, t27592, t27594)
}
