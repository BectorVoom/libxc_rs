//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1405/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1405(t20570: f64, t6945: f64, t1361: f64, t20563: f64, t26288: f64, t107093: f64, t107096: f64, t107100: f64, t107102: f64, t107105: f64, t107107: f64, t107109: f64, t107112: f64, t107115: f64, t107118: f64, t107120: f64, t80826: f64, t91206: f64, t97315: f64, t97347: f64, t97363: f64, t97367: f64, t97372: f64) -> f64 {
    let t107123 = t6945 * t20570;
    let t107126 = t26288 * t1361 * t20563;
    let t107131 = t107093 / 256.0_f64 + 0.10093189023535097714e-3_f64 * t97315 - t107096 / 4.0_f64 - 0.12111826828242117256e-2_f64 * t107100 - t107102 / 64.0_f64 - 0.94875976821229918508e-2_f64 * t91206 - t107105 / 128.0_f64 + 5.0_f64 / 128.0_f64 * t107107 - 5.0_f64 / 64.0_f64 * t107109 - 0.12111826828242117256e-2_f64 * t107112 + 0.12111826828242117256e-2_f64 * t107115 - 0.20186378047070195427e-3_f64 * t107118 - t107120 / 48.0_f64 - 0.12111826828242117256e-2_f64 * t97347 - t80826 - t107123 / 1536.0_f64 + 0.25434836339308446237e-1_f64 * t107126 - 7.0_f64 / 768.0_f64 * t97363 - 0.20186378047070195427e-3_f64 * t97367 + 0.10093189023535097714e-3_f64 * t97372;
    t107131
}
