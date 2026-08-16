//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 970/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk970(t1070: f64, t6651: f64, t3363: f64, t6654: f64, t1271: f64, t3366: f64, t1277: f64, t6661: f64, t826: f64, t1276: f64, t1289: f64, t1266: f64, param_eta: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11041 = t6651 * t1070;
    let t11043 = t6654 * t3363;
    let t11045 = t1271 * t3366;
    let t11046 = 2.0_f64 / 3.0_f64 * t11045;
    let t11047 = t1070 * t1277;
    let t11048 = t6661 * t11047;
    let t11050 = t3366 * t826;
    let t11051 = t1276 * t11050;
    let t11052 = 4.0_f64 / 3.0_f64 * t11051;
    let t11053 = t1070 * t1289;
    let t11054 = t1276 * t11053;
    let t11056 = param_eta * t1266;
    (t11041, t11043, t11045, t11046, t11047, t11048, t11050, t11051, t11052, t11053, t11054, t11056)
}
