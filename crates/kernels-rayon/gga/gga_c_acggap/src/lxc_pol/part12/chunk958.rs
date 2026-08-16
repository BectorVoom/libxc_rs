//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 958/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk958(t1101: f64, t1983: f64, t30827: f64, t7586: f64, t1181: f64, t3346: f64, t599: f64, t7493: f64, t3378: f64, t7584: f64, t7588: f64, t2074: f64, t30797: f64) -> (f64, f64, f64, f64, f64) {
    let t31693 = t30827 * t7586 * t1983 * t1101;
    let t31697 = t7493 * t1181 * t599 * t3346;
    let t31699 = t3378 * t7584;
    let t31700 = t31699 * t7588;
    let t31702 = t30797 * t2074;
    (t31693, t31697, t31699, t31700, t31702)
}
