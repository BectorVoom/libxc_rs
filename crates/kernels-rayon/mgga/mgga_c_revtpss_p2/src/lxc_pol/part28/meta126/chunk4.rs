//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 719/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk719(t198: f64, t207: f64, t2392: f64, t2393: f64, t2394: f64, t2400: f64, t2402: f64, t2403: f64, t2404: f64, t2408: f64, t2411: f64, t2416: f64, t2430: f64, t2569: f64, t2614: f64, t2617: f64, t2832: f64, t765: f64, t775: f64, t892: f64) -> f64 {
    let t2836 = -t198 * t207 * t2408 * t2411 + t198 * t207 * t2832 * t892 + 6.0_f64 * t198 * t2393 * t2394 + 3.0_f64 * t198 * t2430 * t765 + 6.0_f64 * t2403 * t2404 * t775 + t2392 + t2400 + t2402 + t2416 - t2569 + t2614 + t2617;
    t2836
}
