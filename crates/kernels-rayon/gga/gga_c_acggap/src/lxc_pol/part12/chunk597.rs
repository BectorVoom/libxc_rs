//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 597/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk597(t1165: f64, t1532: f64, t4199: f64, t945: f64, t1541: f64, t3375: f64, t1545: f64, t3379: f64, t2450: f64, t3402: f64, t1090: f64, t1181: f64, t530: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4452 = t1165 * t1532 * t4199;
    let t4456 = t1165 * t1532 * t945;
    let t4459 = t3375 * t1541;
    let t4462 = 0.17149607247227894789e-2_f64 * t3379 * t1545;
    let t4463 = t2450 * t3402;
    let t4465 = t1181 * t530 * t1090;
    (t4452, t4456, t4459, t4462, t4463, t4465)
}
