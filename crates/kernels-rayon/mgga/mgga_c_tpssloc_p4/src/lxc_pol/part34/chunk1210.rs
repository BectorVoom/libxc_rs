//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1210/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1210(t107093: f64, t107096: f64, t107100: f64, t107102: f64, t107105: f64, t107107: f64, t107109: f64, t107112: f64, t107115: f64, t107118: f64, t107120: f64, t107123: f64, t107126: f64, t84514: f64, t91206: f64, t97315: f64, t97347: f64, t97363: f64, t97367: f64, t97372: f64) -> f64 {
    let t107822 = t107093 / 128.0_f64 + 0.20186378047070195427e-3_f64 * t97315 - t107096 / 2.0_f64 - 0.24223653656484234512e-2_f64 * t107100 - t107102 / 32.0_f64 - 0.18975195364245983701e-1_f64 * t91206 - t107105 / 64.0_f64 + 5.0_f64 / 64.0_f64 * t107107 - 5.0_f64 / 32.0_f64 * t107109 - 0.24223653656484234512e-2_f64 * t107112 + 0.24223653656484234512e-2_f64 * t107115 - 0.40372756094140390853e-3_f64 * t107118 - t107120 / 24.0_f64 - 0.24223653656484234513e-2_f64 * t97347 - t84514 - t107123 / 768.0_f64 + 0.50869672678616892474e-1_f64 * t107126 - 7.0_f64 / 384.0_f64 * t97363 - 0.40372756094140390854e-3_f64 * t97367 + 0.20186378047070195427e-3_f64 * t97372;
    t107822
}
