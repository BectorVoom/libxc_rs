//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2593/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2593(t10368: f64, t56: f64, t1518: f64, t670: f64, t1921: f64, t5789: f64, t1913: f64, t5808: f64, t22532: f64, t575: f64, t21661: f64, t602: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t60311 = t56 * t10368;
    let t60595 = t670 * t1518;
    let t60620 = t5789 * t1921;
    let t60624 = t1913 * t5808;
    let t60629 = t22532 * t575;
    let t60670 = t21661 * t602;
    (t60311, t60595, t60620, t60624, t60629, t60670)
}
