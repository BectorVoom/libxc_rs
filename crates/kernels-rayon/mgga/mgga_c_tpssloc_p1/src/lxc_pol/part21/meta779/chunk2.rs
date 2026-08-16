//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2704/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2704(t12189: f64, t6375: f64, t40138: f64, t6396: f64, t12283: f64, t19951: f64, t19991: f64, t40281: f64, t12407: f64, t12429: f64, t16224: f64, t16225: f64, t16305: f64, t16306: f64, t16311: f64, t16366: f64, t16370: f64, t16394: f64, t19871: f64, t19921: f64, t19926: f64, t19976: f64, t19981: f64, t19989: f64, t3783: f64, t3803: f64, t3805: f64, t5246: f64, t5303: f64, t53973: f64, t54013: f64, t54162: f64, t54202: f64) -> f64 {
    let t56953 = t12189 * t6375;
    let t56959 = t40138 * t6396;
    let t56961 = t12283 * t19951;
    let t56963 = t12283 * t19991;
    let t56993 = t40281 * t6396;
    let t56996 = -35.0_f64 / 216.0_f64 * t56953 - 5.0_f64 / 192.0_f64 * t3803 * t16224 * t16225 * t19989 - 7.0_f64 / 288.0_f64 * t56959 - 7.0_f64 / 288.0_f64 * t56961 - 7.0_f64 / 288.0_f64 * t56963 + t3803 * t3805 * t19871 * t12407 / 768.0_f64 + t54162 * t5303 / 192.0_f64 + t16394 * t16366 / 192.0_f64 + t16394 * t16370 / 384.0_f64 + t3803 * t16305 * t16306 * t19989 / 192.0_f64 + t5246 * t54013 * t16311 * t53973 / 128.0_f64 - 5.0_f64 / 64.0_f64 * t3783 * t19921 + 5.0_f64 / 192.0_f64 * t3783 * t19926 - t12429 * t19976 / 1536.0_f64 + t12429 * t19951 / 192.0_f64 - 5.0_f64 / 384.0_f64 * t12429 * t19981 + 119.0_f64 / 1728.0_f64 * t56993 + 7.0_f64 / 144.0_f64 * t54202;
    t56996
}
