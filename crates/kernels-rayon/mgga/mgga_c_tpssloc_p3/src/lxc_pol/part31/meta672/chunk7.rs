//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2020/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2020(t93644: f64, t93645: f64, t93646: f64, t97236: f64, t97238: f64, t97240: f64, t97242: f64, t97244: f64, t97247: f64, t97249: f64, t97251: f64, t97253: f64, t97255: f64, t97257: f64, t97259: f64, t97261: f64, t97263: f64, t97266: f64) -> f64 {
    let t102663 = 0.16149102437656156341e-2_f64 * t97236 - 0.33913115119077928317e-1_f64 * t97238 + 7.0_f64 / 576.0_f64 * t97240 - t97242 / 768.0_f64 - t97244 / 768.0_f64 - t97247 / 768.0_f64 - t97249 / 384.0_f64 - t97251 / 384.0_f64 + 7.0_f64 / 1152.0_f64 * t97253 - 5.0_f64 / 32.0_f64 * t97255 + 5.0_f64 / 96.0_f64 * t97257 - t97259 / 768.0_f64 + 7.0_f64 / 288.0_f64 * t97261 + 7.0_f64 / 144.0_f64 * t97263 - t97266 / 192.0_f64 + t93644 + t93645 - t93646;
    t102663
}
