//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2014/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2014(t99041: f64, t99044: f64, t99050: f64, t93001: f64, t95673: f64, t95674: f64, t95675: f64, t99046: f64, t99048: f64, t99052: f64, t99054: f64, t99056: f64) -> f64 {
    let t103301 = 0.22866142996303859718e-3_f64 * t99041;
    let t103302 = 0.40656002247428262579e-4_f64 * t99044;
    let t103305 = 35.0_f64 / 108.0_f64 * t99050;
    let t103310 = t103301 + t103302 + t99046 / 4.0_f64 + t99048 / 8.0_f64 - t95673 - t103305 + 0.34299214494455789578e-2_f64 * t99052 + 0.68598428988911579156e-2_f64 * t99054 + 0.51448821741683684367e-2_f64 * t99056 - t95674 + t95675 - 0.24390552529390783699e-2_f64 * t93001;
    t103310
}
