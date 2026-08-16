//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2488/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2488(t13951: f64, t2713: f64, t3964: f64, t1413: f64, t46835: f64, t48698: f64, t1873: f64, t46651: f64, t13910: f64, t808: f64, t9736: f64, t550: f64, t9794: f64) -> (f64, f64, f64, f64, f64) {
    let t49008 = t3964 * t2713 * t13951;
    let t49012 = t46835 * t1413 * t48698;
    let t49030 = t46651 * t1873;
    let t49056 = t9736 * t808 * t13910;
    let t49057 = 0.30492001685571196935e-4_f64 * t49056;
    let t49068 = t9794 * t550;
    (t49008, t49012, t49030, t49057, t49068)
}
