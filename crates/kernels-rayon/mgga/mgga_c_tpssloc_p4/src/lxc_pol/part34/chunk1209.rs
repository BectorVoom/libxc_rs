//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1209/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1209(t107063: f64, t107065: f64, t107067: f64, t107070: f64, t107074: f64, t107077: f64, t107084: f64, t107086: f64, t107088: f64, t107090: f64, t91149: f64, t91167: f64, t97219: f64, t97238: f64, t97240: f64, t97253: f64, t97261: f64, t97263: f64, t97283: f64) -> f64 {
    let t107802 = t107063 / 64.0_f64 + t107065 / 128.0_f64 + t107067 / 64.0_f64 - 7.0_f64 / 48.0_f64 * t97219 + t107070 / 64.0_f64 - 0.10173934535723378495e0_f64 * t97238 + 7.0_f64 / 192.0_f64 * t97240 - t107074 / 256.0_f64 + 7.0_f64 / 384.0_f64 * t97253 + 5.0_f64 / 64.0_f64 * t107077 + 7.0_f64 / 96.0_f64 * t97261 + 7.0_f64 / 48.0_f64 * t97263 - 119.0_f64 / 288.0_f64 * t91149 - 35.0_f64 / 96.0_f64 * t97283 - 0.67826230238155856633e-1_f64 * t91167 - t107084 / 768.0_f64 - t107086 / 256.0_f64 - t107088 / 256.0_f64 - t107090 / 128.0_f64;
    t107802
}
