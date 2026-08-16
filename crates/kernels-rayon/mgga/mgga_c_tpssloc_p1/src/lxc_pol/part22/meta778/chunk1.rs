//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2664/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2664(t1824: f64, t6414: f64, t119: f64, t1315: f64, t16101: f64, t16224: f64, t16305: f64, t16321: f64, t19994: f64, t20433: f64, t20570: f64, t210: f64, t221: f64, t3778: f64, t3783: f64, t3803: f64, t3807: f64, t40168: f64, t5301: f64, t5308: f64, t54614: f64, t6415: f64, t6420: f64, t6427: f64, t74355: f64, t74389: f64, t74393: f64, t74395: f64, t74401: f64, t74403: f64, t74405: f64) -> (f64, f64) {
    let t74415 = t6414 * t1824;
    let t74428 = -t3778 * t20570 / 3072.0_f64 - 3.0_f64 / 4.0_f64 * t16101 * t221 * t74389 - 7.0_f64 / 16.0_f64 * t74393 + 7.0_f64 / 144.0_f64 * t74395 - t1315 * t210 * t119 * t74355 / 48.0_f64 - 7.0_f64 / 768.0_f64 * t74401 + 7.0_f64 / 1152.0_f64 * t74403 - 35.0_f64 / 384.0_f64 * t74405 + 5.0_f64 / 256.0_f64 * t16321 * t6427 - 5.0_f64 / 128.0_f64 * t3783 * t20433 - 15.0_f64 / 128.0_f64 * t54614 * t40168 * t5301 * t19994 + t3803 * t16305 * t74415 * t3807 / 256.0_f64 - 5.0_f64 / 256.0_f64 * t3803 * t16224 * t6415 * t5308 - 5.0_f64 / 256.0_f64 * t3803 * t16224 * t6420 * t5308;
    (t74415, t74428)
}
