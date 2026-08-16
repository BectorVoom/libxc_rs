//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1081/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1081(t467: f64, t9476: f64, t1298: f64, t560: f64, t469: f64, t5506: f64, t157: f64, t1914: f64, t406: f64, t1814: f64, t33795: f64, t615: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t38559 = t9476 * t467;
    let t38563 = t1298 * t560;
    let t38573 = t469 * t5506;
    let t38635 = t1914 * t406 * t157;
    let t38647 = t1814 * t406 * t157;
    let t38662 = t615 * t33795;
    (t38559, t38563, t38573, t38635, t38647, t38662)
}
