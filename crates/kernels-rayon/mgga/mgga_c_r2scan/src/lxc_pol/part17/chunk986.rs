//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 986/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk986(t11550: f64, t3262: f64, t3465: f64, t11523: f64, t3469: f64, t11657: f64, t11660: f64, t10700: f64, t10713: f64, t11641: f64, t11644: f64, t11647: f64, t11650: f64, t11652: f64, t11655: f64, t11663: f64) -> (f64, f64, f64, f64, f64) {
    let t12109 = t3262 * t3465 * t11550;
    let t12110 = 3.0_f64 / 4.0_f64 * t12109;
    let t12111 = t11523 * t3469;
    let t12112 = t12111 / 4.0_f64;
    let t12120 = 0.23115257973478049502e0_f64 * t11657;
    let t12121 = 0.46574606203128791246e-1_f64 * t11660;
    let t12123 = t10700 - 0.87327386630866483588e-2_f64 * t11641 - 0.13099107994629972538e-1_f64 * t11644 - 0.13099107994629972538e-1_f64 * t11647 - 0.52396431978519890152e-1_f64 * t11650 + 0.43663693315433241794e-2_f64 * t11652 + 0.43663693315433241794e-2_f64 * t11655 - t12120 - t12121 + t10713 + 0.86682217400542685632e-1_f64 * t11663;
    (t12109, t12110, t12111, t12112, t12123)
}
