//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1169/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1169(t1165: f64, t2068: f64, t39596: f64, t7351: f64, t31350: f64, t5737: f64, t7337: f64, t8480: f64, t8902: f64, t30698: f64, t38789: f64, t604: f64) -> (f64, f64, f64, f64) {
    let t40241 = t2068 * t1165 * t7351 * t39596;
    let t40243 = t31350 * t5737;
    let t40246 = t7337 * t8480 * t8902;
    let t40251 = t30698 * t1165 * t604 * t38789;
    (t40241, t40243, t40246, t40251)
}
