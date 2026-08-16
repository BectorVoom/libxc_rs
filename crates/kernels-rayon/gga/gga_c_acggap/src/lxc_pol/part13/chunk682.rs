//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 682/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk682(t301: f64, t7381: f64, t7380: f64, t7312: f64, t7313: f64, t7317: f64, t7319: f64, t7328: f64, t7331: f64, t7333: f64, t7340: f64, t7344: f64, t7350: f64, t7354: f64, t7358: f64, t7362: f64, t7366: f64, t7368: f64, t7373: f64, t7376: f64, t7379: f64) -> (f64, f64, f64) {
    let t7382 = t7381 * t301;
    let t7383 = t7380 * t7382;
    let t7384 = t7383 / 32.0_f64;
    let t7385 = t7312 - t7313 / 96.0_f64 + t7317 + t7319 - t7328 + t7331 + t7333 / 16.0_f64 + 0.10718504529517434243e-2_f64 * t7340 + 0.42874018118069736972e-3_f64 * t7344 + t7350 - 0.94344276868812456204e-3_f64 * t7354 - 0.15724046144802076034e-2_f64 * t7358 + 0.62896184579208304136e-3_f64 * t7362 - 0.31448092289604152068e-3_f64 * t7366 + 0.85748036236139473944e-3_f64 * t7368 + t7373 - t7376 + t7379 - t7384;
    (t7382, t7383, t7385)
}
