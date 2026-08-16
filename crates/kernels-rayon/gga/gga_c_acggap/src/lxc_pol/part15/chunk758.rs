//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 758/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk758(t2069: f64, t8480: f64, t2068: f64, t137: f64, t1524: f64, t1089: f64, t1459: f64, t598: f64, t355: f64, t513: f64, t7458: f64, t1980: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8481 = t8480 * t2069;
    let t8482 = t2068 * t8481;
    let t8484 = t137 * t1524;
    let t8486 = t1089 * t1459 * t8484;
    let t8487 = t598 * t8486;
    let t8489 = t355 * t513;
    let t8491 = t7458 * t1459 * t8489;
    let t8492 = t1980 * t8491;
    (t8481, t8482, t8484, t8486, t8487, t8489, t8491, t8492)
}
