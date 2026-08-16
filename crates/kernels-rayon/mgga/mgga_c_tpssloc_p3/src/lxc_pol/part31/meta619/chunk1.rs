//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1870/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1870(t1352: f64, t22633: f64, t6976: f64, t96964: f64, t96951: f64, t19743: f64, t3807: f64, t1992: f64, t20014: f64, t1351: f64, t550: f64, t6434: f64) -> (f64, f64, f64, f64, f64) {
    let t96967 = t22633 * t6976 * t96964 * t1352;
    let t96972 = t22633 * t6976 * t96951 * t1352;
    let t96976 = t22633 * t6976 * t19743 * t3807;
    let t96979 = t1992 * t6976 * t20014;
    let t96986 = t1992 * t6976 * t6434 * t1351 * t550;
    (t96967, t96972, t96976, t96979, t96986)
}
