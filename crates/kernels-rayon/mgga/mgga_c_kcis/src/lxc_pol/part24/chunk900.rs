//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 900/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk900(t6467: f64, t743: f64, t1056: f64, t18677: f64, t18672: f64, t345: f64, t18653: f64, t1154: f64, t18648: f64, t18657: f64, t1079: f64, t104: f64, t111: f64, t120: f64, t19423: f64, t19425: f64, t19427: f64, t19430: f64, t19433: f64, t19436: f64, t19438: f64, t19440: f64, t4858: f64, t4881: f64) -> f64 {
    let t19442 = t743 * t6467;
    let t19444 = t1056 * t18677;
    let t19447 = t345 * t18672;
    let t19450 = t345 * t18653;
    let t19453 = t1154 * t18648;
    let t19456 = t345 * t18657;
    let t19459 = t1079 * t18677;
    let t19462 = t1056 * t18672;
    let t19465 = t345 * t18648;
    let t19468 = t1056 * t18657;
    let t19471 = 0.4684e-2_f64 * t19423 - 0.15613333333333333333e-2_f64 * t19425 - 0.3513e-2_f64 * t104 * t19427 + 0.7925e-3_f64 * t111 * t19430 + 0.50413125e-5_f64 * t120 * t19433 + 0.15684083333333333333e-4_f64 * t19436 - 0.13208333333333333333e-2_f64 * t19438 + 0.88055555555555555555e-3_f64 * t19440 - 0.117630625e-4_f64 * t19442 + 0.7026e-2_f64 * t104 * t19444 + 0.1171e-2_f64 * t104 * t19447 - 0.7026e-2_f64 * t104 * t19450 + 0.78066666666666666667e-3_f64 * t104 * t19453 + 0.4684e-2_f64 * t4858 * t19456 - 0.10082625e-4_f64 * t120 * t19459 - 0.672175e-5_f64 * t120 * t19462 + 0.22405833333333333333e-5_f64 * t120 * t19465 - 0.26887e-4_f64 * t4881 * t19468;
    t19471
}
