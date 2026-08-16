//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 596/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk596(t1165: f64, t540: f64, t955: f64, t535: f64, t1181: f64, t530: f64, t1532: f64, t1016: f64, t513: f64) -> (f64, f64, f64, f64, f64) {
    let t4402 = t1165 * t540 * t955;
    let t4405 = t535 * t955;
    let t4406 = t1181 * t4405;
    let t4410 = t1165 * t530 * t955;
    let t4414 = t1165 * t1532 * t955;
    let t4417 = t1016 * t513;
    (t4402, t4406, t4410, t4414, t4417)
}
