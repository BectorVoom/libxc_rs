//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 631/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk631(t2291: f64, t38: f64, t45: f64, t631: f64, t78: f64, t57: f64, t635: f64, t81: f64, t2251: f64, t2258: f64, t633: f64, t637: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2292 = t38 * t2291;
    let t2297 = t631 * t45;
    let t2299 = 1.0_f64 / t78 / t2297;
    let t2304 = t635 * t57;
    let t2306 = 1.0_f64 / t81 / t2304;
    let t2311 = 28.0_f64 / 9.0_f64 * t2299 * t2251 - 4.0_f64 / 3.0_f64 * t633 * t2258 + 28.0_f64 / 9.0_f64 * t2306 * t2251 + 4.0_f64 / 3.0_f64 * t637 * t2258;
    (t2292, t2297, t2299, t2304, t2306, t2311)
}
