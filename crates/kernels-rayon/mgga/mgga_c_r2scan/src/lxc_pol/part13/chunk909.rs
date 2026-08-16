//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 909/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk909(t322: f64, t8397: f64, t2394: f64, t833: f64, t1013: f64, t1299: f64, t1295: f64, t829: f64, t1292: f64, t1300: f64, t2397: f64, t327: f64, t6693: f64, t834: f64) -> (f64, f64, f64, f64, f64) {
    let t324 = 0.0_f64 < t322;
    let t8398 = piecewise3(t324, 0.0_f64, t8397);
    let t8401 = t2394 * t833;
    let t8404 = t1013 * t1299;
    let t8409 = t1013 * t1295;
    let t8412 = t2394 * t829;
    let t8415 = t1013 * t1292;
    let t8420 = -0.64e0_f64 * t8398 * t327 - 0.256e1_f64 * t8401 * t829 - 0.384e1_f64 * t8404 * t1295 - 0.128e1_f64 * t2397 * t1292 - 0.384e1_f64 * t6693 * t8409 - 0.256e1_f64 * t1300 * t8412 - 0.128e1_f64 * t1300 * t8415 - 0.64e0_f64 * t834 * t8398;
    (t8398, t8409, t8412, t8415, t8420)
}
