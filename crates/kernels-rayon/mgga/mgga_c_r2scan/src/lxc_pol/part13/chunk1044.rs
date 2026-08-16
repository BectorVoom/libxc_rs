//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1044/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1044(t1277: f64, t3366: f64, t6661: f64, t6651: f64, t11056: f64, t1271: f64, t1276: f64, t1289: f64, t6100: f64, t819: f64, t826: f64, t11153: f64, t1348: f64, param_eta: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t37055 = t6661 * t3366 * t1277;
    let t37063 = t6651 * t3366;
    let t37066 = t1271 * t11056;
    let t37069 = t1276 * t3366 * t1289;
    let t37074 = param_eta * t6100;
    let t37075 = t819 * t37074;
    let t37076 = 154.0_f64 / 27.0_f64 * t37075;
    let t37078 = t1276 * t11056 * t826;
    let t37199 = t1348 * t11153;
    (t37055, t37063, t37066, t37069, t37076, t37078, t37199)
}
