//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1872/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1872(t12369: f64, t19743: f64, t22633: f64, t22897: f64, t562: f64, t6330: f64, t1307: f64, t26446: f64, t90591: f64, t1992: f64, t20018: f64, t6976: f64) -> (f64, f64, f64, f64) {
    let t97007 = t22633 * t22897 * t19743 * t12369;
    let t97011 = t562 * t6330;
    let t97014 = t90591 * t26446 * t97011 * t1307;
    let t97017 = t1992 * t6976 * t20018;
    (t97007, t97011, t97014, t97017)
}
