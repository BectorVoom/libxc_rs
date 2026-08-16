//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 918/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk918(t2067: f64, t3073: f64, t3088: f64, t1165: f64, t15758: f64, t604: f64, t1181: f64, t599: f64, t1089: f64, t2079: f64, t30052: f64, t368: f64) -> (f64, f64, f64, f64, f64) {
    let t31562 = t3073 * t2067;
    let t31567 = t3088 * t2067;
    let t31570 = t31567 * t1165 * t604 * t15758;
    let t31593 = t31567 * t1181 * t599 * t15758;
    let t31597 = t2079 * t1089 * t368 * t30052;
    (t31562, t31567, t31570, t31593, t31597)
}
