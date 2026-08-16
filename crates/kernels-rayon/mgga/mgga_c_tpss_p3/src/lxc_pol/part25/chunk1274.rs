//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1274/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1274(t1215: f64, t60730: f64, t339: f64, t5719: f64, t790: f64, t2376: f64, t5726: f64, t1250: f64, t31297: f64, t522: f64, t2436: f64, t580: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t60731 = t60730 * t1215;
    let t60738 = t339 * t5719 * t790;
    let t60749 = t339 * t5726 * t2376;
    let t60750 = t60749 * t1250;
    let t60811 = t31297 * t522;
    let t60960 = t2436 * t580;
    (t60731, t60738, t60749, t60750, t60811, t60960)
}
