//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2708/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2708(t1307: f64, t5286: f64, t1351: f64, t6387: f64, t12283: f64, t19894: f64, t12240: f64, t1352: f64, t16224: f64, t16233: f64, t16271: f64, t16275: f64, t16305: f64, t16394: f64, t1825: f64, t19871: f64, t19956: f64, t19994: f64, t210: f64, t3719: f64, t3733: f64, t3803: f64, t3807: f64, t40124: f64, t40126: f64, t40145: f64, t5246: f64, t5248: f64, t54014: f64, t54068: f64, t54153: f64, t54293: f64, t54295: f64, t54533: f64, t54535: f64, t6374: f64, t6394: f64) -> (f64, f64) {
    let t57086 = t1307 * t5286;
    let t57091 = t6387 * t1351;
    let t57127 = t12283 * t19894;
    let t57133 = t3803 * t16305 * t1825 * t57086 / 192.0_f64 + t3803 * t16305 * t57091 * t3807 / 384.0_f64 + t5246 * t5248 * t19871 * t12240 / 512.0_f64 + t5246 * t5248 * t19956 * t12240 / 1536.0_f64 + 595.0_f64 / 5184.0_f64 * t40124 - 119.0_f64 / 13824.0_f64 * t40126 - t16394 * t16271 / 768.0_f64 - t16394 * t16275 / 1536.0_f64 - 595.0_f64 / 5184.0_f64 * t40145 + t3733 * t210 * t6374 * t3719 / 16.0_f64 + 7.0_f64 / 1152.0_f64 * t54293 + 7.0_f64 / 2304.0_f64 * t54295 - 119.0_f64 / 3456.0_f64 * t54533 + 7.0_f64 / 2304.0_f64 * t54535 + t3803 * t16305 * t54153 * t6394 / 384.0_f64 + t16233 * t16305 * t54014 * t54068 / 64.0_f64 + 35.0_f64 / 288.0_f64 * t57127 - 5.0_f64 / 384.0_f64 * t3803 * t16224 * t19994 * t1352;
    (t57086, t57133)
}
