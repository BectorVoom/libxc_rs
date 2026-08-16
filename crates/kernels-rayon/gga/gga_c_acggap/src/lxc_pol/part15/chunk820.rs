//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 820/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk820(t495: f64, t560: f64, t1734: f64, t469: f64, t157: f64, t524: f64, t556: f64, t1907: f64, t615: f64, t1745: f64, t589: f64, t137: f64, t1713: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9476 = t495 * t560;
    let t9480 = t469 * t1734;
    let t9508 = t556 * t524 * t157;
    let t9517 = t615 * t1907;
    let t9522 = t589 * t1745;
    let t9529 = t137 * t1713;
    (t9476, t9480, t9508, t9517, t9522, t9529)
}
