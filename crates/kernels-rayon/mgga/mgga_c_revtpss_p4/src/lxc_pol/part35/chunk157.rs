//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 157/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk157(t30: f64, t33: f64, t512: f64, t521: f64, t187: f64, t520: f64, t513: f64, t199: f64, t516: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t522 = t512 * t521;
    let t524 = 0.19751673498613801407e-1_f64 * t520 * t187;
    let t525 = t513 * t513;
    let t526 = piecewise3(t31, t199, t525);
    let t527 = t516 * t516;
    let t528 = piecewise3(t34, t199, t527);
    let t530 = t526 / 2.0_f64 + t528 / 2.0_f64;
    (t522, t524, t525, t527, t530)
}
