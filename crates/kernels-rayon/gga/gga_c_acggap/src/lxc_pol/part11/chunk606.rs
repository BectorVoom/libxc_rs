//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 606/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk606(t1314: f64, t3282: f64, t1318: f64, t1298: f64, t145: f64, t301: f64, t960: f64, t1567: f64, t372: f64, t1131: f64, t530: f64, t1327: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4571 = t3282 * t1314;
    let t4574 = t3282 * t1318;
    let t4577 = t145 * t1298;
    let t4578 = t4577 * t301;
    let t4579 = t960 * t4578;
    let t4582 = t1567 * t372;
    let t4583 = t960 * t4582;
    let t4586 = t530 * t1131;
    let t4587 = t960 * t4586;
    let t4590 = t3282 * t1327;
    (t4571, t4574, t4578, t4579, t4582, t4583, t4586, t4587, t4590)
}
