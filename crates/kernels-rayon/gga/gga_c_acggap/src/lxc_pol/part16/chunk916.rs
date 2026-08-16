//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 916/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk916(t31498: f64, t2092: f64, t7780: f64, t154: f64, t2096: f64, t31035: f64, t3036: f64, t597: f64, t137: f64, t3037: f64, t1089: f64, t1095: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31499 = 0.62896184579208304135e-3_f64 * t31498;
    let t31505 = t7780 * t2092;
    let t31508 = t31035 * t154 * t2096;
    let t31509 = 0.52805208333333333333e0_f64 * t31508;
    let t31520 = t3036 * t597;
    let t31521 = t137 * t3037;
    let t31524 = t31520 * t1089 * t1095 * t31521;
    (t31499, t31505, t31509, t31520, t31521, t31524)
}
