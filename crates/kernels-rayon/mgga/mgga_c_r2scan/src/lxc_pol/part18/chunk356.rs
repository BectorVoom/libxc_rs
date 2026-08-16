//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 356/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk356(t1422: f64, t89: f64, t377: f64, t431: f64, t430: f64, t68: f64, t63: f64, t437: f64, t438: f64, t1399: f64, t1402: f64, t1404: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1423 = t1422 * t89;
    let t1424 = 32.0_f64 * t1423;
    let t1428 = t377 * t431;
    let t1432 = t430 * t68;
    let t1433 = 1.0_f64 / t1432;
    let t1434 = t63 * t1433;
    let t1435 = t437 * t437;
    let t1436 = t1435 * t438;
    let t1441 = 0.68863333333333333333e0_f64 * t1399;
    let t1442 = 0.14025833333333333333e0_f64 * t1402;
    let t1443 = 0.28051666666666666667e0_f64 * t1404;
    (t1424, t1428, t1433, t1434, t1435, t1436, t1441, t1442, t1443)
}
