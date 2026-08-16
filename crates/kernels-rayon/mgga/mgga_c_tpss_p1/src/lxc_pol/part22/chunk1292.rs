//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1292/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1292(t1250: f64, t60749: f64, t18464: f64, t3354: f64, t18480: f64, t5570: f64, t31297: f64, t522: f64, t2436: f64, t580: f64, t1699: f64, t8202: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t60750 = t60749 * t1250;
    let t60752 = t18464 * t3354;
    let t60778 = t18480 * t5570;
    let t60811 = t31297 * t522;
    let t60960 = t2436 * t580;
    let t61024 = t1699 * t8202;
    (t60750, t60752, t60778, t60811, t60960, t61024)
}
