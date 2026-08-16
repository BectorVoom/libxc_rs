//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1177/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1177(t1967: f64, t9687: f64, t1165: f64, t31562: f64, t38778: f64, t604: f64, t2068: f64, t38827: f64, t38647: f64, t7346: f64, t8480: f64, t8896: f64) -> (f64, f64, f64, f64, f64) {
    let t40222 = t1967 * t9687;
    let t40226 = t31562 * t1165 * t604 * t38778;
    let t40230 = t2068 * t1165 * t604 * t38827;
    let t40234 = t2068 * t1165 * t604 * t38647;
    let t40237 = t7346 * t8480 * t8896;
    (t40222, t40226, t40230, t40234, t40237)
}
