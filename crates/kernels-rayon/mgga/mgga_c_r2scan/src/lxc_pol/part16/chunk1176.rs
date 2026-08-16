//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1176/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1176(t3295: f64, t9526: f64, t27067: f64, t3610: f64, t37600: f64, t39429: f64, t39438: f64, t39440: f64, t39444: f64, t39446: f64, t39459: f64, t39482: f64, t41395: f64, t41397: f64) -> f64 {
    let t43057 = t3295 * t9526;
    let t43061 = t27067 * t3610;
    let t43064 = 0.54878743191129263322e-1_f64 * t43057 + 0.31147743054556651236e-1_f64 * t39429 + t39438 - 0.95219938395347901944e-2_f64 * t39440 - t39444 + t39446 - t39459 - t41395 - t41397 + 0.43663693315433241792e-2_f64 * t43061 - t37600 + 0.31147743054556651236e-1_f64 * t39482;
    t43064
}
