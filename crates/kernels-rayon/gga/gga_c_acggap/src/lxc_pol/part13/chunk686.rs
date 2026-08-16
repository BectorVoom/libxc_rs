//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 686/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk686(t1165: f64, t604: f64, t945: f64, t7413: f64, t955: f64, t2068: f64, t599: f64, t1181: f64, t2067: f64, t3360: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7415 = t1165 * t604 * t945;
    let t7416 = t7413 * t7415;
    let t7419 = t1165 * t604 * t955;
    let t7420 = t2068 * t7419;
    let t7422 = t599 * t955;
    let t7423 = t1181 * t7422;
    let t7424 = t2068 * t7423;
    let t7426 = t3360 * t2067;
    (t7415, t7416, t7419, t7420, t7422, t7423, t7424, t7426)
}
