//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1019/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1019(t322: f64, t3675: f64, t856: f64, t1338: f64, t3678: f64, t11893: f64, t1348: f64, t11145: f64, t11148: f64, t11157: f64, t11162: f64, t11920: f64, t11924: f64, t11926: f64, t11960: f64, t11991: f64, t2438: f64, t330: f64, t3413: f64, t3420: f64, t352: f64, t3643: f64, t3645: f64, t837: f64, t838: f64, t855: f64, t9760: f64) -> (f64, f64, f64, f64, f64) {
    let t323 = t322 <= 0.0_f64;
    let t331 = t322 <= 0.25e1_f64;
    let t332 = 0.25e1_f64 < t322;
    let t11993 = t3675 * t856;
    let t12002 = t1338 * t3678;
    let t12005 = piecewise3(t332, t11893, 0.0_f64);
    let t12009 = t1348 * t3678;
    let t12019 = piecewise5(t323, t330 * t3643 * t837 + t11920 * t330 + t11924 * t330 + t11926 * t330 + t3645 * t838, t331, t11960 + t11991, -0.63e1_f64 * t3420 * t11993 - 0.21e1_f64 * t11145 * t3675 - 0.945e1_f64 * t11148 * t11993 - 0.21e1_f64 * t3413 * t9760 - 0.21e1_f64 * t12002 * t2438 - 0.105e1_f64 * t855 * t12005 * t352 - 0.1575e1_f64 * t12009 * t2438 - 0.1575e1_f64 * t11157 * t3675 - 0.1575e1_f64 * t3420 * t9760 - 0.23625e1_f64 * t11162 * t11993);
    (t11993, t12002, t12005, t12009, t12019)
}
