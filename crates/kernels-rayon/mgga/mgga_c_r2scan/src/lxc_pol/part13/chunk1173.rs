//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1173/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1173(t39960: f64, t546: f64, t10729: f64, t25172: f64, t3332: f64, t6165: f64, t25177: f64, t7614: f64, t11659: f64, t6395: f64, t38055: f64, t38056: f64, t38062: f64, t40059: f64, t40064: f64, t40068: f64, t40070: f64, t40073: f64) -> f64 {
    let t40075 = t546 * t39960;
    let t40076 = t40075 * t10729;
    let t40077 = 0.47609969197673950972e-2_f64 * t40076;
    let t40081 = t6165 * t3332 * t25172;
    let t40084 = t7614 * t3332 * t25177;
    let t40086 = t6395 * t11659;
    let t40087 = 0.46574606203128791246e-1_f64 * t40086;
    let t40088 = 0.86682217400542685632e-1_f64 * t40059 + 0.87327386630866483584e-2_f64 * t40064 + 0.26198215989259945076e-1_f64 * t40068 - 0.59512461497092438715e-1_f64 * t40070 + 0.13002332610081402845e0_f64 * t40073 + t40077 - t38055 - 0.11557628986739024751e0_f64 * t38056 + 0.46574606203128791246e-1_f64 * t38062 - 0.13099107994629972538e-1_f64 * t40081 - 0.5239643197851989015e-1_f64 * t40084 - t40087;
    t40088
}
