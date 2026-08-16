//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 972/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk972(t322: f64, t11059: f64, t3370: f64, t833: f64, t1074: f64, t1299: f64, t1295: f64, t829: f64, t1292: f64, t1300: f64, t327: f64, t3373: f64, t6693: f64, t834: f64) -> (f64, f64, f64, f64) {
    let t324 = 0.0_f64 < t322;
    let t11060 = piecewise3(t324, 0.0_f64, t11059);
    let t11063 = t3370 * t833;
    let t11066 = t1074 * t1299;
    let t11071 = t1074 * t1295;
    let t11074 = t3370 * t829;
    let t11077 = t1074 * t1292;
    let t11082 = -0.64e0_f64 * t11060 * t327 - 0.256e1_f64 * t11063 * t829 - 0.384e1_f64 * t11066 * t1295 - 0.128e1_f64 * t3373 * t1292 - 0.384e1_f64 * t6693 * t11071 - 0.256e1_f64 * t1300 * t11074 - 0.128e1_f64 * t1300 * t11077 - 0.64e0_f64 * t834 * t11060;
    (t11060, t11063, t11066, t11082)
}
