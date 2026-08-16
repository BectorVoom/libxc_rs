//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1123/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1123(t2068: f64, t4680: f64, t9636: f64, t1181: f64, t599: f64, t6069: f64, t2041: f64, t5590: f64, t5594: f64, t1165: f64, t5645: f64, t604: f64, t8463: f64) -> (f64, f64, f64, f64, f64) {
    let t39551 = t2068 * t4680 * t9636;
    let t39555 = t2068 * t1181 * t599 * t6069;
    let t39557 = t2041 * t5590;
    let t39559 = t2041 * t5594;
    let t39563 = t8463 * t1165 * t604 * t5645;
    (t39551, t39555, t39557, t39559, t39563)
}
