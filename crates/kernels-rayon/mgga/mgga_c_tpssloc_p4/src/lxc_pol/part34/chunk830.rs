//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 830/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk830(t4049: f64, t5396: f64, t20215: f64, t95: f64, t5415: f64, t1449: f64, t5480: f64, t9398: f64, t4059: f64, t5484: f64, t103: f64, t100: f64, t104: f64, t1447: f64, t1450: f64, t20312: f64, t5475: f64, t5481: f64, t5485: f64, t92: f64, tau1: f64) -> f64 {
    let t20315 = t4049 * t5396;
    let t20318 = 3.0_f64 * t20215;
    let t20319 = t95 * t20318;
    let t20322 = tau1 * t5415;
    let t20331 = t5480 * t1449;
    let t20332 = t9398 * t20331;
    let t20335 = t4059 * t5484;
    let t20338 = -t20318;
    let t20339 = t103 * t20338;
    let t20342 = -10.0_f64 / 27.0_f64 * t92 * t20312 + 10.0_f64 / 3.0_f64 * t92 * t20315 + 5.0_f64 / 3.0_f64 * t92 * t20319 - 440.0_f64 / 27.0_f64 * t20322 * t104 + 200.0_f64 / 9.0_f64 * t5475 * t1450 - 50.0_f64 / 9.0_f64 * t1447 * t5481 - 25.0_f64 / 3.0_f64 * t1447 * t5485 - 10.0_f64 / 27.0_f64 * t100 * t20332 + 10.0_f64 / 3.0_f64 * t100 * t20335 + 5.0_f64 / 3.0_f64 * t100 * t20339;
    t20342
}
