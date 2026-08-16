//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 993/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk993(t1494: f64, t7329: f64, t1498: f64, t30716: f64, t500: f64, t1411: f64, t1983: f64, t7585: f64, t7586: f64, t1165: f64, t4555: f64, t604: f64, t7493: f64) -> (f64, f64, f64, f64, f64) {
    let t35039 = t7329 * t1494;
    let t35041 = t7329 * t1498;
    let t35043 = t30716 * t500;
    let t35051 = t7585 * t7586 * t1983 * t1411;
    let t35055 = t7493 * t1165 * t604 * t4555;
    (t35039, t35041, t35043, t35051, t35055)
}
