//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 141/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk141(t406: f64, t409: f64, t412: f64, t416: f64, t439: f64, t300: f64, t424: f64, t426: f64, t435: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t444 = 0.51785e1_f64 * t409 + 0.905775e0_f64 * t406 + 0.1100325e0_f64 * t412 + 0.1241775e0_f64 * t416;
    let t447 = 1.0_f64 + 0.29608749977793437516e2_f64 / t444;
    let t448 = f64::ln(t447);
    let t449 = t439 * t448;
    let t452 = t300 * (-0.310907e-1_f64 * t426 * t435 + t424 - 0.19751673498613801407e-1_f64 * t449);
    let t454 = 0.19751673498613801407e-1_f64 * t300 * t449;
    let t456 = 1.0_f64 + 0.25e-1_f64 * t406;
    let t458 = 1.0_f64 + 0.4445e-1_f64 * t406;
    let t459 = 1.0_f64 / t458;
    let t460 = t456 * t459;
    (t444, t447, t448, t452, t454, t456, t458, t459, t460)
}
