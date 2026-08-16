//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2165/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2165(t22756: f64, t6422: f64, t22783: f64, t6431: f64, t1831: f64, t91160: f64, t19815: f64, t6951: f64, t1369: f64, t91136: f64, t91138: f64, t91141: f64, t97236: f64, t97238: f64, t97240: f64, t97242: f64, t97244: f64, t97247: f64, t97249: f64, t97251: f64, t97253: f64, t97255: f64, t97257: f64) -> f64 {
    let t97259 = t22756 * t6422;
    let t97261 = t22783 * t6431;
    let t97263 = t91160 * t1831;
    let t97265 = t19815 * t6951;
    let t97266 = t97265 * t1369;
    let t97268 = 0.80745512188280781708e-3_f64 * t97236 - 0.16956557559538964158e-1_f64 * t97238 + 7.0_f64 / 1152.0_f64 * t97240 - t97242 / 1536.0_f64 - t97244 / 1536.0_f64 - t97247 / 1536.0_f64 - t97249 / 768.0_f64 - t97251 / 768.0_f64 + 7.0_f64 / 2304.0_f64 * t97253 - 5.0_f64 / 64.0_f64 * t97255 + 5.0_f64 / 192.0_f64 * t97257 - t97259 / 1536.0_f64 + 7.0_f64 / 576.0_f64 * t97261 + 7.0_f64 / 288.0_f64 * t97263 - t97266 / 384.0_f64 + t91136 + t91138 - t91141;
    t97268
}
