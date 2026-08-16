//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1273/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1273(t6072: f64, t689: f64, t7014: f64, t6049: f64, t106128: f64, t25375: f64, t18805: f64, t93261: f64, t213: f64, t29636: f64, t105945: f64, t7063: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t106286 = t689 * t7014 * t6072;
    let t106316 = t689 * t7014 * t6049;
    let t106318 = t25375 * t106128;
    let t106326 = t93261 * t18805;
    let t106353 = t213 * t29636;
    let t106387 = t7063 * t105945;
    (t106286, t106316, t106318, t106326, t106353, t106387)
}
