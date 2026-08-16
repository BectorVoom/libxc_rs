//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 882/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk882(t10426: f64, t4594: f64, t4582: f64, t10283: f64, t10361: f64, t10364: f64, t10367: f64, t10370: f64, t10372: f64, t10377: f64, t10378: f64, t10381: f64, t10385: f64, t10388: f64, t10390: f64, t10394: f64, t10398: f64, t10403: f64, t10405: f64, t10410: f64, t10413: f64, t10415: f64, t10419: f64, t10424: f64, t3070: f64, t3073: f64, t3130: f64, t350: f64, t378: f64, t973: f64) -> (f64, f64, f64) {
    let t10427 = t10426 * t4594;
    let t10428 = t4582 * t10427;
    let t10431 = t10361 * t378 / 3072.0_f64 + t973 * t10364 / 72.0_f64 - t10367 * t378 / 192.0_f64 + t10370 / 1536.0_f64 + t10372 / 864.0_f64 + t10377 - t973 * t10378 / 48.0_f64 + t10381 / 54.0_f64 + t10385 - 77.0_f64 / 162.0_f64 * t10283 * t350 + 11.0_f64 / 108.0_f64 * t10388 + t10390 * t3073 / 768.0_f64 + t3070 * t10394 / 1536.0_f64 + t3070 * t10398 / 1536.0_f64 + t10403 * t10405 / 768.0_f64 + 5.0_f64 / 4608.0_f64 * t3070 * t10410 - t10413 * t10415 / 1536.0_f64 - t3070 * t10419 / 768.0_f64 + t10424 / 1152.0_f64 + t3130 * t10428 / 512.0_f64;
    (t10427, t10428, t10431)
}
