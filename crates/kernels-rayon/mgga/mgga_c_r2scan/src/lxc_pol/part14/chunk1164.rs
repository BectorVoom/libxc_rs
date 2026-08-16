//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1164/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1164(t3578: f64, t494: f64, t97: f64, t113: f64, t11505: f64, t1543: f64, t2867: f64, t2259: f64, t3582: f64, t3446: f64, t3453: f64, t7133: f64) -> (f64, f64, f64, f64, f64) {
    let t40276 = t97 * t3578 * t494;
    let t40282 = t97 * t11505 * t113;
    let t40285 = t2867 * t1543;
    let t40289 = t3582 * t2259;
    let t40294 = t3446 * t3453 * t7133;
    (t40276, t40282, t40285, t40289, t40294)
}
