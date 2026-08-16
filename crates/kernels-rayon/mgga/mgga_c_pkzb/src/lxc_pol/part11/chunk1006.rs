//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1006/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1006(t11070: f64, t6013: f64, t10993: f64, t2970: f64, t6027: f64, t10979: f64, t133: f64, t793: f64, t11028: f64, t6048: f64, t11053: f64, t1138: f64, t290: f64, t2969: f64, t2984: f64, t3680: f64, t3686: f64, t3689: f64, t6009: f64, t6026: f64, t6047: f64, t7871: f64, t7879: f64, t791: f64, t9695: f64, t9707: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11071 = t11070 * t6013;
    let t11076 = t2970 * t10993;
    let t11079 = t11070 * t6027;
    let t11088 = t10979 * t133;
    let t11089 = t11088 * t793;
    let t11092 = t2970 * t11028;
    let t11095 = t11070 * t6048;
    let t11100 = 0.39512695097613069591e1_f64 * t6009 * t11071 + 0.39512695097613069591e1_f64 * t7871 * t3680 + 0.39512695097613069591e1_f64 * t2969 * t11076 - 0.39512695097613069591e1_f64 * t6026 * t11079 + 0.19756347548806534796e1_f64 * t9707 * t1138 + 0.19756347548806534796e1_f64 * t2984 * t3686 - 0.19756347548806534796e1_f64 * t7879 * t3689 + 0.65854491829355115987e0_f64 * t791 * t11089 - 0.19756347548806534796e1_f64 * t9695 * t11092 + 0.65854491829355115987e0_f64 * t6047 * t11095 + 0.65854491829355115987e0_f64 * t290 * t11053;
    (t11071, t11076, t11079, t11088, t11089, t11092, t11095, t11100)
}
