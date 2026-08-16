//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 352/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk352(t322: f64, t1339: f64, t1348: f64, t1306: f64, t1308: f64, t1336: f64, t1338: f64, t1343: f64, t352: f64, t855: f64, t410: f64, t458: f64) -> (f64, f64) {
    let t323 = t322 <= 0.0_f64;
    let t331 = t322 <= 0.25e1_f64;
    let t1349 = t1348 * t1339;
    let t1353 = piecewise5(t323, t1306 + t1308, t331, t1336, -0.21e1_f64 * t1338 * t1339 * t352 - 0.105e1_f64 * t855 * t1343 * t352 - 0.1575e1_f64 * t1349 * t352);
    let t1355 = t410 * t458;
    let t1356 = 8.0_f64 * t1355;
    (t1353, t1356)
}
