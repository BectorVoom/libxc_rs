//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1016/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1016(t1181: f64, t23736: f64, t7351: f64, t7575: f64, t30268: f64, t8956: f64, t21099: f64, t599: f64, t7337: f64, t2068: f64, t20935: f64, t604: f64) -> (f64, f64, f64, f64) {
    let t34105 = t7575 * t1181 * t7351 * t23736;
    let t34107 = t30268 * t8956;
    let t34111 = t7337 * t1181 * t599 * t21099;
    let t34115 = t2068 * t1181 * t604 * t20935;
    (t34105, t34107, t34111, t34115)
}
