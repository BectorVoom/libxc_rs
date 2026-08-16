//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 951/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk951(t11031: f64, t502: f64, t818: f64, t826: f64, t1275: f64, t263: f64, t1271: f64, t3366: f64, t1276: f64, t1266: f64, param_eta: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11032 = 11.0_f64 / 9.0_f64 * t11031;
    let t11033 = t502 * t818;
    let t11034 = t11033 * t826;
    let t11036 = t263 * t1275;
    let t11045 = t1271 * t3366;
    let t11050 = t3366 * t826;
    let t11051 = t1276 * t11050;
    let t11056 = param_eta * t1266;
    (t11032, t11033, t11034, t11036, t11045, t11050, t11051, t11056)
}
