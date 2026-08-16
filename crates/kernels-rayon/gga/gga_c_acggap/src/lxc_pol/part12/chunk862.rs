//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 862/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk862(t1160: f64, t7432: f64, t7365: f64, t1165: f64, t16548: f64, t604: f64, t7346: f64, t1992: f64, t7585: f64, t7842: f64, t930: f64, t2067: f64, t4180: f64) -> (f64, f64, f64, f64, f64) {
    let t30105 = t1160 * t7432;
    let t30106 = t30105 * t7365;
    let t30110 = t7346 * t1165 * t604 * t16548;
    let t30118 = t7585 * t7842 * t1992 * t930;
    let t30120 = t4180 * t2067;
    (t30105, t30106, t30110, t30118, t30120)
}
