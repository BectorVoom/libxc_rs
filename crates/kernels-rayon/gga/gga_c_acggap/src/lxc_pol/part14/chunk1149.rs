//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1149/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1149(t1181: f64, t26554: f64, t7351: f64, t7426: f64, t2068: f64, t8480: f64, t8778: f64, t26214: f64, t604: f64, t30543: f64, t9653: f64, t5532: f64, t7564: f64, t8600: f64) -> (f64, f64, f64, f64, f64) {
    let t39876 = t7426 * t1181 * t7351 * t26554;
    let t39879 = t2068 * t8480 * t8778;
    let t39883 = t2068 * t1181 * t604 * t26214;
    let t39885 = t30543 * t9653;
    let t39889 = t7564 * t1181 * t8600 * t5532;
    (t39876, t39879, t39883, t39885, t39889)
}
