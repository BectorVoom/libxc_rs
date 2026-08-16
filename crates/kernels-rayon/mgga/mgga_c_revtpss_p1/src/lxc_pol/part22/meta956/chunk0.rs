//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3200/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3200(t1464: f64, t5789: f64, t18177: f64, t575: f64, t1921: f64, t1913: f64, t5808: f64, t22532: f64, t2327: f64, t5876: f64, t2319: f64, t5883: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t60616 = t5789 * t1464;
    let t60618 = t18177 * t575;
    let t60620 = t5789 * t1921;
    let t60624 = t1913 * t5808;
    let t60629 = t22532 * t575;
    let t60650 = t5876 * t2327;
    let t60656 = t2319 * t5883;
    (t60616, t60618, t60620, t60624, t60629, t60650, t60656)
}
