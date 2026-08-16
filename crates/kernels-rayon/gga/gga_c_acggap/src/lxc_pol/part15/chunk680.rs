//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 680/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk680(t1111: f64, t1165: f64, t7351: f64, t7426: f64, t1964: f64, t592: f64, t2066: f64) -> (f64, f64, f64, f64) {
    let t7428 = t1165 * t7351 * t1111;
    let t7429 = t7426 * t7428;
    let t7431 = t592 * t1964;
    let t7432 = t7431 * t2066;
    (t7428, t7429, t7431, t7432)
}
