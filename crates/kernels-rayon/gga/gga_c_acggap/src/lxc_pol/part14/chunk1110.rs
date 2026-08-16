//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1110/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1110(t1181: f64, t604: f64, t6203: f64, t7575: f64, t1165: f64, t6209: f64, t7351: f64, t20417: f64, t2068: f64, t2073: f64, t31346: f64, t5932: f64) -> (f64, f64, f64, f64) {
    let t39382 = t7575 * t1181 * t604 * t6203;
    let t39386 = t7575 * t1165 * t7351 * t6209;
    let t39389 = t2068 * t20417 * t2073;
    let t39391 = t31346 * t5932;
    (t39382, t39386, t39389, t39391)
}
