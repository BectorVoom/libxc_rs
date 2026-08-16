//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1643/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1643(t3498: f64, t5205: f64, t1196: f64, t12485: f64, t1756: f64, t3524: f64, t3531: f64, t5198: f64, t12361: f64, t5068: f64, t12243: f64, t5109: f64) -> (f64, f64, f64, f64, f64) {
    let t16639 = t5205 * t3498;
    let t16641 = 0.35089341735807877242e1_f64 * t1196 * t16639;
    let t16642 = t12485 * t1756;
    let t16643 = t16642 * t3524;
    let t16645 = 0.10389515463408878255e3_f64 * t1196 * t16643;
    let t16647 = 0.23392894490538584828e1_f64 * t3531 * t5198;
    let t16649 = 4.0_f64 * t12361 * t5068;
    let t16651 = 0.32163958997385070134e2_f64 * t12243 * t5109;
    (t16641, t16645, t16647, t16649, t16651)
}
