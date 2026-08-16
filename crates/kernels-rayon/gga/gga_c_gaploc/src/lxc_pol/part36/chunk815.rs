//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 815/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk815(t41572: f64, t2902: f64, t9243: f64, t2798: f64, t9588: f64, t10624: f64, t2355: f64, t10295: f64, t19933: f64, t24215: f64, t3366: f64, t13001: f64, t1377: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41573 = 4.0_f64 * t41572;
    let t41574 = t9243 * t2902;
    let t41575 = t2798 * t9588;
    let t41576 = t2355 * t10624;
    let t41577 = 2.0_f64 * t41576;
    let t41579 = 12.0_f64 * t19933 * t10295;
    let t41581 = 4.0_f64 * t24215 * t3366;
    let t41582 = t1377 * t13001;
    (t41573, t41574, t41575, t41577, t41579, t41581, t41582)
}
