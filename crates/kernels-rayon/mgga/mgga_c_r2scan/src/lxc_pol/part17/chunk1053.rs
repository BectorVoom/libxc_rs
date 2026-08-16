//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1053/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1053(t6100: f64, t819: f64, t11056: f64, t1276: f64, t826: f64, t113: f64, t3268: f64, t97: f64, t10666: f64, t1561: f64, t3261: f64, t122: f64, t874: f64, param_eta: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t37074 = param_eta * t6100;
    let t37075 = t819 * t37074;
    let t37078 = t1276 * t11056 * t826;
    let t37271 = t97 * t3268 * t113;
    let t37282 = t97 * t10666 * t113;
    let t37327 = t97 * t3261 * t1561;
    let t37355 = t874 * t122;
    (t37075, t37078, t37271, t37282, t37327, t37355)
}
