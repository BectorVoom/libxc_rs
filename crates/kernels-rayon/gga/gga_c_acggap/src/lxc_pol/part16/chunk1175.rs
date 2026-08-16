//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1175/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1175(t1181: f64, t5959: f64, t604: f64, t7575: f64, t5964: f64, t1859: f64, t1992: f64, t30154: f64, t7586: f64, t1164: f64, t9685: f64, t2068: f64, t2069: f64) -> (f64, f64, f64, f64) {
    let t40196 = t7575 * t1181 * t604 * t5959;
    let t40200 = t7575 * t1181 * t604 * t5964;
    let t40204 = t30154 * t7586 * t1992 * t1859;
    let t40206 = t1164 * t9685;
    let t40208 = t2068 * t40206 * t2069;
    (t40196, t40200, t40204, t40208)
}
