//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 964/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk964(t1987: f64, t2875: f64, t2866: f64, t1972: f64, t2865: f64, t730: f64, t1116: f64, t5754: f64, t237: f64, t2826: f64, t732: f64, t1995: f64, t2860: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7552 = 0.34631718211362927518e2_f64 * t1987 * t2875;
    let t7554 = 0.23392894490538584828e1_f64 * t1987 * t2866;
    let t7555 = t2865 * t1972;
    let t7557 = 0.11696447245269292414e1_f64 * t730 * t7555;
    let t7559 = 0.5848223622634646207e0_f64 * t5754 * t1116;
    let t7560 = t237 * t2826;
    let t7562 = 0.11696447245269292414e1_f64 * t7560 * t732;
    let t7564 = 0.5848223622634646207e0_f64 * t2860 * t1995;
    (t7552, t7554, t7555, t7557, t7559, t7560, t7562, t7564)
}
