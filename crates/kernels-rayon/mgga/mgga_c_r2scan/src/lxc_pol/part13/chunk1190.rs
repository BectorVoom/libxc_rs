//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1190/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1190(t3578: f64, t494: f64, t97: f64, t11004: f64, t113: f64, t11505: f64, t11012: f64, t1543: f64, t2867: f64, t10610: f64, t3263: f64, t2259: f64, t3582: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40276 = t97 * t3578 * t494;
    let t40278 = 5.0_f64 / 8.0_f64 * t40276 * t11004;
    let t40282 = t97 * t11505 * t113;
    let t40284 = 3.0_f64 / 2.0_f64 * t40282 * t11012;
    let t40285 = t2867 * t1543;
    let t40288 = 3.0_f64 / 2.0_f64 * t10610 * t3263 * t40285;
    let t40289 = t3582 * t2259;
    (t40276, t40278, t40282, t40284, t40288, t40289)
}
