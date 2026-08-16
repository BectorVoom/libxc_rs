//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 615/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk615(t1532: f64, t4711: f64, t1181: f64, t1545: f64, t3431: f64, t1524: f64, t322: f64, t1095: f64, t398: f64, t384: f64, t1089: f64, t1444: f64, t429: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4712 = t1532 * t4711;
    let t4713 = t1181 * t4712;
    let t4716 = t3431 * t1545;
    let t4718 = t1524 * t322;
    let t4720 = t398 * t1095 * t4718;
    let t4722 = 0.85748036236139473944e-3_f64 * t384 * t4720;
    let t4724 = t1089 * t429 * t1444;
    (t4713, t4716, t4718, t4720, t4722, t4724)
}
