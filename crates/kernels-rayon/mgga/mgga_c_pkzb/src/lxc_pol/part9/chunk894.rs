//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 894/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk894(t6597: f64, t942: f64, t2422: f64, t2430: f64, t2454: f64, t411: f64, t415: f64, t6536: f64, t6548: f64, t6552: f64, t938: f64, t952: f64) -> (f64, f64) {
    let t6598 = t942 * t6597;
    let t6601 = 0.65854491829355115987e0_f64 * t6536 * t415 - 0.19756347548806534796e1_f64 * t2422 * t952 + 0.39512695097613069591e1_f64 * t938 * t2430 - 0.19756347548806534796e1_f64 * t938 * t2454 - 0.39512695097613069591e1_f64 * t411 * t6548 + 0.39512695097613069591e1_f64 * t411 * t6552 - 0.65854491829355115987e0_f64 * t411 * t6598;
    (t6598, t6601)
}
