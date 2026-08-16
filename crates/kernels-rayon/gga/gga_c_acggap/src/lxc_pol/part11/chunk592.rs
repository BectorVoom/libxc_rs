//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 592/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk592(t535: f64, t930: f64, t1181: f64, t1165: f64, t540: f64, t1490: f64, t330: f64, t3740: f64, t527: f64, t1017: f64, t495: f64, t1089: f64, t1459: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4330 = t535 * t930;
    let t4331 = t1181 * t4330;
    let t4335 = t1165 * t540 * t930;
    let t4339 = 7.0_f64 / 144.0_f64 * t330 * t1490;
    let t4340 = t3740 * t527;
    let t4342 = t495 * t1017;
    let t4344 = t1089 * t1459 * t4342;
    (t4331, t4335, t4339, t4340, t4342, t4344)
}
