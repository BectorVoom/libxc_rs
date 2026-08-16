//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 862/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk862(t13277: f64, t1646: f64, t3203: f64, t3316: f64, t3202: f64, t3200: f64, t1800: f64, t2829: f64, t2845: f64, t4554: f64, t1804: f64, t3210: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13278 = 0.33163888888888888888e-2_f64 * t13277;
    let t13280 = t3203 * t1646 * t3316;
    let t13281 = t3202 * t13280;
    let t13282 = t3200 * t13281;
    let t13284 = t1800 * t2829;
    let t13285 = t3202 * t13284;
    let t13286 = t3200 * t13285;
    let t13288 = t1800 * t2845;
    let t13289 = t3202 * t13288;
    let t13290 = t4554 * t13289;
    let t13292 = t1804 * t2829;
    let t13293 = t3210 * t13292;
    (t13278, t13280, t13282, t13284, t13286, t13288, t13290, t13292, t13293)
}
