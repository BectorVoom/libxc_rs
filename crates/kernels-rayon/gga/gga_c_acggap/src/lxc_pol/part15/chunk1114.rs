//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1114/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1114(t1089: f64, t4643: f64, t598: f64, t8484: f64, t1980: f64, t38798: f64, t7458: f64, t5676: f64, t570: f64, t6171: f64, t1750: f64, t31824: f64) -> (f64, f64, f64, f64, f64) {
    let t39240 = t598 * t1089 * t4643 * t8484;
    let t39243 = t1980 * t7458 * t38798;
    let t39254 = t570 * t5676;
    let t39256 = t570 * t6171;
    let t39262 = t31824 * t1750;
    (t39240, t39243, t39254, t39256, t39262)
}
