//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1176/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1176(t4199: f64, t9494: f64, t13123: f64, t9885: f64, t9722: f64, t1409: f64, t707: f64, t9862: f64, t9467: f64, t9713: f64, t1471: f64, t31: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t46208 = t4199 * t9494;
    let t46278 = t13123 * t9885;
    let t46302 = t4199 * t9722;
    let t46369 = t707 * t9862 * t1409;
    let t46371 = t13123 * t9467;
    let t46376 = t4199 * t9713;
    let t46387 = t31 * t1471;
    (t46208, t46278, t46302, t46369, t46371, t46376, t46387)
}
