//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 633/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk633(t157: f64, t922: f64, t1165: f64, t1532: f64, t4183: f64, t3451: f64, t1541: f64, t3372: f64, t1298: f64, t372: f64, t1089: f64, t1095: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4919 = t157 * t922;
    let t4921 = t1165 * t1532 * t4919;
    let t4925 = t1165 * t1532 * t4183;
    let t4926 = t3451 * t4925;
    let t4928 = t3372 * t1541;
    let t4930 = t1298 * t372;
    let t4932 = t1089 * t1095 * t4930;
    (t4921, t4925, t4926, t4928, t4930, t4932)
}
