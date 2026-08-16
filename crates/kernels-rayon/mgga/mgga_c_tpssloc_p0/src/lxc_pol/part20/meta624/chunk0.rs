//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2246/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2246(t10109: f64, t1527: f64, t13036: f64, t225: f64, t2678: f64, t829: f64, t828: f64, t9632: f64, t1519: f64, t9971: f64, t13336: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t46488 = t10109 * t1527;
    let t46508 = t13036 * t225;
    let t46511 = t829 * t2678;
    let t46519 = t9632 * t828;
    let t46524 = t9971 * t1519;
    let t46528 = t13336 * t68;
    (t46488, t46508, t46511, t46519, t46524, t46528)
}
