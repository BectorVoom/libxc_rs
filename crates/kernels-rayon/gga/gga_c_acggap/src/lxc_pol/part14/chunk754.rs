//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 754/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk754(t1444: f64, t604: f64, t1181: f64, t7575: f64, t1449: f64, t7351: f64, t7564: f64, t1541: f64, t7647: f64, t1456: f64, t2001: f64, t1165: f64, t1421: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8445 = t604 * t1444;
    let t8446 = t1181 * t8445;
    let t8447 = t7575 * t8446;
    let t8449 = t7351 * t1449;
    let t8450 = t1181 * t8449;
    let t8451 = t7564 * t8450;
    let t8453 = t7647 * t1541;
    let t8455 = t2001 * t1456;
    let t8458 = t1165 * t604 * t1421;
    (t8446, t8447, t8450, t8451, t8453, t8455, t8458)
}
