//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2486/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2486(t1409: f64, t707: f64, t9862: f64, t13123: f64, t9467: f64, t4199: f64, t9713: f64, t1471: f64, t31: f64, t4211: f64, t9874: f64, t13119: f64, t2663: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46369 = t707 * t9862 * t1409;
    let t46371 = t13123 * t9467;
    let t46376 = t4199 * t9713;
    let t46387 = t31 * t1471;
    let t46433 = t4211 * t9874;
    let t46435 = t13119 * t2663;
    (t46369, t46371, t46376, t46387, t46433, t46435)
}
