//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 821/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk821(t301: f64, t525: f64, t599: f64, t1181: f64, t7337: f64, t372: f64, t604: f64, t2068: f64, t8402: f64, t2297: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8901 = t525 * t301;
    let t8902 = t599 * t8901;
    let t8903 = t1181 * t8902;
    let t8904 = t7337 * t8903;
    let t8906 = t525 * t372;
    let t8907 = t604 * t8906;
    let t8908 = t1181 * t8907;
    let t8909 = t2068 * t8908;
    let t8911 = t599 * t8402;
    let t8912 = t1181 * t8911;
    let t8913 = t2068 * t8912;
    let t8915 = t2297 * t301;
    (t8901, t8902, t8903, t8904, t8906, t8907, t8908, t8909, t8911, t8912, t8913, t8915)
}
