//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 901/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk901(t1: f64, t203: f64, t3157: f64, t174: f64, t31009: f64, t31010: f64, t172: f64, t420: f64, t435: f64, t7746: f64, t993: f64, t130: f64, t1964: f64) -> (f64, f64, f64, f64) {
    let t31013 = t3157 * t1 * t203;
    let t31015 = t31009 * t31010 * t174 * t31013;
    let t31016 = 0.3572834843172478081e-3_f64 * t31015;
    let t31020 = t31009 * t420 * t172 * t435 * t31013;
    let t31021 = 0.52413487149340253445e-3_f64 * t31020;
    let t31022 = t7746 * t993;
    let t31023 = 0.60023625365297631762e-2_f64 * t31022;
    let t31035 = t130 * t1964;
    (t31016, t31021, t31023, t31035)
}
