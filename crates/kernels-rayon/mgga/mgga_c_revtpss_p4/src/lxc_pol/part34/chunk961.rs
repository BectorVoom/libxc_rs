//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 961/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk961(t45: f64, t190: f64, t22688: f64, t10439: f64, t4546: f64, t5966: f64, t18540: f64, t18545: f64, t18547: f64, t14363: f64, t22671: f64, t4328: f64, t5825: f64, t633: f64, t766: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t151 = t45 <= zeta_threshold;
    let t23121 = t190 * t22688;
    let t23123 = 24.0_f64 * t10439 * t23121;
    let t23124 = t4546 * t5966;
    let t23127 = 36.0_f64 * t18540;
    let t23128 = 12.0_f64 * t18545;
    let t23129 = 24.0_f64 * t18547;
    let t23130 = 0.32530743900905219526e-1_f64 * t14363;
    let t23138 = piecewise3(t151, 0.0_f64, 8.0_f64 / 27.0_f64 * t633 * t22688 - 2.0_f64 / 3.0_f64 * t4328 * t5825 + 2.0_f64 / 3.0_f64 * t766 * t22671);
    (t23123, t23124, t23127, t23128, t23129, t23130, t23138)
}
