//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 720/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk720(t1340: f64, t9419: f64, t2491: f64, t745: f64, t9368: f64, t169: f64, t2552: f64, t164: f64, t2538: f64, t729: f64, t2556: f64, t9283: f64, t9286: f64, t9289: f64, t9292: f64, t9296: f64, t9298: f64, t9300: f64, t9303: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9421 = 0.10389515463408878255e3_f64 * t1340 * t9419;
    let t9425 = t2491 * t9368 * t745;
    let t9427 = 0.35089341735807877242e1_f64 * t1340 * t9425;
    let t9432 = 1.0_f64 / t2552 / t169;
    let t9433 = t164 * t9432;
    let t9434 = t2538 * t729;
    let t9435 = t9434 * t2556;
    let t9446 = -0.47063e1_f64 * t9283 + 0.31375333333333333334e1_f64 * t9286 - 0.36604555555555555556e1_f64 * t9289 - 0.16068111111111111111e1_f64 * t9292 + 0.28051666666666666666e0_f64 * t9296 - 0.56103333333333333332e0_f64 * t9298 - 0.6545388888888888889e0_f64 * t9300 - 0.46308888888888888888e0_f64 * t9303;
    (t9421, t9425, t9427, t9433, t9434, t9435, t9446)
}
