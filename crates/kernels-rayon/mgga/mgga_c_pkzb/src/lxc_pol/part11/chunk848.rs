//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 848/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk848(t8768: f64, t8793: f64, t9120: f64, t9125: f64, t45: f64, t3715: f64, t645: f64, t1116: f64, t7560: f64, t2860: f64, t2870: f64, t1987: f64, t3618: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9127 = t8768 + t8793 + t9120 + t9125;
    let t9128 = t45 * t9127;
    let t9129 = t645 * t3715;
    let t9132 = 0.11696447245269292414e1_f64 * t7560 * t1116;
    let t9134 = 0.11696447245269292414e1_f64 * t2860 * t2870;
    let t9136 = 0.11696447245269292414e1_f64 * t1987 * t3618;
    (t9127, t9128, t9129, t9132, t9134, t9136)
}
