//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1084/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1084(t17912: f64, t31443: f64, t35296: f64, t13299: f64, t31115: f64, t33938: f64, t1181: f64, t20817: f64, t599: f64, t7337: f64, t7433: f64, t8779: f64) -> (f64, f64, f64, f64) {
    let t35298 = t31443 * t17912 * t35296;
    let t35301 = t31115 * t13299 * t33938;
    let t35305 = t7337 * t1181 * t599 * t20817;
    let t35307 = t7433 * t8779;
    (t35298, t35301, t35305, t35307)
}
