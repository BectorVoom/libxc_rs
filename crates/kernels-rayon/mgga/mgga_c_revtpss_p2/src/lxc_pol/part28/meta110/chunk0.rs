//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 670/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk670(t177: f64, t752: f64, t762: f64, t717: f64, t750: f64, t675: f64, t723: f64, t169: f64, t722: f64, t164: f64, t729: f64, t730: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2523 = t752 * t177;
    let t2524 = t2523 * t762;
    let t2525 = 0.11696447245269292414e1_f64 * t2524;
    let t2526 = t717 * t750;
    let t2527 = 2.0_f64 * t2526;
    let t2531 = t675 * t723;
    let t2535 = t722 * t169;
    let t2536 = 1.0_f64 / t2535;
    let t2537 = t164 * t2536;
    let t2538 = t729 * t729;
    let t2539 = t2538 * t730;
    (t2523, t2524, t2525, t2526, t2527, t2531, t2536, t2537, t2538, t2539)
}
