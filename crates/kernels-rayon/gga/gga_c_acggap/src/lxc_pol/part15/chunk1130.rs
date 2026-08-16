//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1130/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1130(t1181: f64, t2068: f64, t599: f64, t6069: f64, t2041: f64, t5590: f64, t5594: f64, t1165: f64, t5645: f64, t604: f64, t8463: f64, t31362: f64, t9589: f64) -> (f64, f64, f64, f64, f64) {
    let t39555 = t2068 * t1181 * t599 * t6069;
    let t39557 = t2041 * t5590;
    let t39559 = t2041 * t5594;
    let t39563 = t8463 * t1165 * t604 * t5645;
    let t39567 = t31362 * t9589;
    (t39555, t39557, t39559, t39563, t39567)
}
