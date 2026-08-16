//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 574/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk574(t1569: f64, t997: f64, t1101: f64, t1165: f64, t540: f64, t3361: f64, t535: f64, t1181: f64, t1111: f64, t4643: f64, t3391: f64, t1532: f64, t4183: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4901 = t997 * t1569;
    let t4904 = t1165 * t540 * t1101;
    let t4906 = 0.34299214494455789578e-2_f64 * t3361 * t4904;
    let t4907 = t535 * t1101;
    let t4908 = t1181 * t4907;
    let t4910 = 0.34299214494455789578e-2_f64 * t3361 * t4908;
    let t4915 = t4643 * t1111;
    let t4916 = t1181 * t4915;
    let t4918 = 0.17149607247227894789e-2_f64 * t3391 * t4916;
    let t4925 = t1165 * t1532 * t4183;
    (t4901, t4904, t4906, t4908, t4910, t4916, t4918, t4925)
}
