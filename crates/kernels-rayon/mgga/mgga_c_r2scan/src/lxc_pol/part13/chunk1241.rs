//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1241/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1241(t322: f64, t40851: f64, t1083: f64, t1085: f64, t1087: f64, t1089: f64, t2412: f64, t3390: f64, t3394: f64, t3398: f64, t3402: f64, t8440: f64, t8463: f64, t8465: f64) -> (f64, f64) {
    let t332 = 0.25e1_f64 < t322;
    let t40893 = piecewise3(t332, 0.0_f64, t40851);
    let t40923 = -0.64e0_f64 * t40893 + 0.1469548921044e3_f64 * t3390 * t2412 + 0.1469548921044e3_f64 * t1083 * t8465 + 0.734774460522e2_f64 * t1083 * t8463 - 0.22988522834472e3_f64 * t3394 * t2412 - 0.22988522834472e3_f64 * t1085 * t8465 - 0.11494261417236e3_f64 * t1085 * t8463 + 0.12405227240928e3_f64 * t3398 * t2412 + 0.12405227240928e3_f64 * t1087 * t8465 + 0.6202613620464e2_f64 * t1087 * t8463 - 0.2177652951264e2_f64 * t3402 * t2412 - 0.2177652951264e2_f64 * t1089 * t8465 - 0.1088826475632e2_f64 * t1089 * t8463 - 0.22988522834472e3_f64 * t1083 * t8440 + 0.18607840861392e3_f64 * t1085 * t8440;
    (t40893, t40923)
}
