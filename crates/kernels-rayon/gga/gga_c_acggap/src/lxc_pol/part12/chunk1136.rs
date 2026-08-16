//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1136/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1136(t30689: f64, t5286: f64, t1165: f64, t2068: f64, t20972: f64, t7351: f64, t1181: f64, t22107: f64, t604: f64, t8463: f64, t4257: f64, t22275: f64, t7493: f64) -> (f64, f64, f64, f64, f64) {
    let t36177 = t30689 * t5286;
    let t36181 = t2068 * t1165 * t7351 * t20972;
    let t36186 = t8463 * t1181 * t604 * t22107;
    let t36190 = t8463 * t1165 * t7351 * t4257;
    let t36194 = t7493 * t1181 * t604 * t22275;
    (t36177, t36181, t36186, t36190, t36194)
}
