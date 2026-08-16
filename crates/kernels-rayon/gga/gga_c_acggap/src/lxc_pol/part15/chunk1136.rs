//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1136/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1136(t1323: f64, t507: f64, t7436: f64, t1181: f64, t30806: f64, t5824: f64, t599: f64, t5969: f64, t7493: f64, t1839: f64, t1983: f64, t7585: f64, t7586: f64) -> (f64, f64, f64, f64) {
    let t39661 = t7436 * t507 * t1323;
    let t39665 = t30806 * t1181 * t599 * t5824;
    let t39669 = t7493 * t1181 * t599 * t5969;
    let t39673 = t7585 * t7586 * t1983 * t1839;
    (t39661, t39665, t39669, t39673)
}
