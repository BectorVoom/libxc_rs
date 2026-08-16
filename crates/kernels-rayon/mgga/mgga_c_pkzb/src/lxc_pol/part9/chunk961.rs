//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 961/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk961(t1980: f64, t7531: f64, t730: f64, t1976: f64, t2848: f64, t2874: f64, t1999: f64, t2860: f64, t1147: f64, t6065: f64, t1987: f64, t2870: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7532 = t7531 * t1980;
    let t7534 = 0.10389515463408878255e3_f64 * t730 * t7532;
    let t7535 = t1976 * t2848;
    let t7536 = t7535 * t2874;
    let t7538 = 0.34631718211362927518e2_f64 * t730 * t7536;
    let t7540 = 0.17315859105681463759e2_f64 * t2860 * t1999;
    let t7543 = t1147 * t6065;
    let t7548 = 0.11696447245269292414e1_f64 * t1987 * t2870;
    (t7532, t7534, t7535, t7536, t7538, t7540, t7543, t7548)
}
