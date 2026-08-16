//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 960/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk960(t7799: f64, t8545: f64, t8491: f64, t30402: f64, t31309: f64, t525: f64, t7325: f64, t31362: f64, t8783: f64, t4959: f64, t7647: f64, t30148: f64, t5606: f64, t7585: f64, t7842: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34056 = t7799 * t8545;
    let t34059 = t7799 * t8491;
    let t34068 = t31309 * t30402 * t7325 * t525;
    let t34081 = t31362 * t8783;
    let t34082 = 0.15724046144802076034e-2_f64 * t34081;
    let t34091 = t7647 * t4959;
    let t34092 = 0.17149607247227894789e-2_f64 * t34091;
    let t34095 = t7585 * t7842 * t30148 * t5606;
    (t34056, t34059, t34068, t34082, t34092, t34095)
}
