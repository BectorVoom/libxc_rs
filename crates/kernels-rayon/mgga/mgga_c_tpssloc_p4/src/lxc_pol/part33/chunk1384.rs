//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1384/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1384(t20565: f64, t6952: f64, t20556: f64, t6945: f64, t1827: f64, t97246: f64, t26233: f64, t6417: f64, t20492: f64, t80903: f64, t107063: f64, t107065: f64, t107067: f64, t107070: f64, t107074: f64, t91149: f64, t91167: f64, t97219: f64, t97238: f64, t97240: f64, t97253: f64, t97261: f64, t97263: f64, t97283: f64) -> f64 {
    let t107077 = t6952 * t20565;
    let t107084 = t6945 * t20556;
    let t107086 = t97246 * t1827;
    let t107088 = t26233 * t6417;
    let t107090 = t80903 * t20492;
    let t107092 = t107063 / 128.0_f64 + t107065 / 256.0_f64 + t107067 / 128.0_f64 - 7.0_f64 / 96.0_f64 * t97219 + t107070 / 128.0_f64 - 0.50869672678616892476e-1_f64 * t97238 + 7.0_f64 / 384.0_f64 * t97240 - t107074 / 512.0_f64 + 7.0_f64 / 768.0_f64 * t97253 + 5.0_f64 / 128.0_f64 * t107077 + 7.0_f64 / 192.0_f64 * t97261 + 7.0_f64 / 96.0_f64 * t97263 - 119.0_f64 / 576.0_f64 * t91149 - 35.0_f64 / 192.0_f64 * t97283 - 0.33913115119077928317e-1_f64 * t91167 - t107084 / 1536.0_f64 - t107086 / 512.0_f64 - t107088 / 512.0_f64 - t107090 / 256.0_f64;
    t107092
}
