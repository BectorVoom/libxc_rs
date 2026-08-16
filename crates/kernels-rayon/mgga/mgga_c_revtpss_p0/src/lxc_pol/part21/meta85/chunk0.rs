//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 605/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk605(t1803: f64, t467: f64, t1264: f64, t1715: f64, t247: f64, t1221: f64, t1222: f64, t1235: f64, t1247: f64, t1258: f64, t1261: f64, t1778: f64, t1782: f64, t1786: f64, t1791: f64, t1797: f64, t464: f64, t484: f64) -> (f64, f64, f64) {
    let t1804 = t467 * t1803;
    let t1807 = t1264 * t1715;
    let t1808 = t247 * t1807;
    let t1811 = -t1778 * t464 / 36.0_f64 + t1221 - t1222 * t1782 / 288.0_f64 + 0.21437009059034868486e-3_f64 * t1786 * t484 - 0.21437009059034868486e-3_f64 * t1235 * t1791 + 0.21437009059034868486e-3_f64 * t1247 * t1797 - 0.11433071498151929859e-2_f64 * t1804 * t484 + t1258 - 0.14291339372689912324e-3_f64 * t1261 * t1808;
    (t1804, t1808, t1811)
}
