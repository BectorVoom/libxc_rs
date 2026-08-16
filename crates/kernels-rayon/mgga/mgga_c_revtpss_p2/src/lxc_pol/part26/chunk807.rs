//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 807/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk807(t10380: f64, t38: f64, t2851: f64, t78: f64, t2299: f64, t606: f64, t3361: f64, t81: f64, t2306: f64, t10326: f64, t10356: f64, t2258: f64, t633: f64, t637: f64) -> (f64, f64) {
    let t10381 = t38 * t10380;
    let t10389 = 1.0_f64 / t78 / t2851;
    let t10392 = t2299 * t606;
    let t10398 = 1.0_f64 / t81 / t3361;
    let t10401 = t2306 * t606;
    let t10406 = -280.0_f64 / 27.0_f64 * t10389 * t10356 + 28.0_f64 / 3.0_f64 * t10392 * t2258 - 4.0_f64 / 3.0_f64 * t633 * t10326 + 280.0_f64 / 27.0_f64 * t10398 * t10356 + 28.0_f64 / 3.0_f64 * t10401 * t2258 + 4.0_f64 / 3.0_f64 * t637 * t10326;
    (t10381, t10406)
}
