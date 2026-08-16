//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1132/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1132(t1181: f64, t20590: f64, t599: f64, t7337: f64, t31567: f64, t36019: f64, t1992: f64, t7585: f64, t7586: f64, t8960: f64, t30148: f64, t7842: f64, t8906: f64) -> (f64, f64, f64, f64) {
    let t36111 = t7337 * t1181 * t599 * t20590;
    let t36115 = t31567 * t1181 * t599 * t36019;
    let t36119 = t7585 * t7586 * t1992 * t8960;
    let t36123 = t7585 * t7842 * t30148 * t8906;
    (t36111, t36115, t36119, t36123)
}
