//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2338/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2338(t13278: f64, t5619: f64, t1512: f64, t59281: f64, t1484: f64, t16662: f64, t16872: f64, t16951: f64, t20800: f64, t20904: f64, t20949: f64, t20953: f64, t2618: f64, t2623: f64, t2701: f64, t4119: f64, t41344: f64, t4172: f64, t4236: f64, t46650: f64, t46878: f64, t5527: f64, t5544: f64, t5587: f64, t58576: f64, t776: f64, t820: f64, t843: f64, t9607: f64) -> f64 {
    let t67852 = t13278 * t5619;
    let t67854 = t59281 * t1512;
    let t67865 = -119.0_f64 / 2304.0_f64 * t58576 + t46650 + 5.0_f64 / 256.0_f64 * t4172 * t16951 + t46878 * t5587 / 512.0_f64 - 15.0_f64 / 128.0_f64 * t843 * t9607 * t820 * t5527 * t4119 + 5.0_f64 / 256.0_f64 * t2623 * t20949 + 5.0_f64 / 256.0_f64 * t843 * t2701 * t820 * t4119 * t5544 + 5.0_f64 / 256.0_f64 * t843 * t2701 * t820 * t1484 * t16662 - t2618 * t20953 / 3072.0_f64 + 7.0_f64 / 1536.0_f64 * t67852 + 7.0_f64 / 1536.0_f64 * t67854 + 5.0_f64 / 768.0_f64 * t843 * t2701 * t820 * t20800 * t776 - t41344 * t20904 / 512.0_f64 - t16872 * t4236 / 1024.0_f64;
    t67865
}
