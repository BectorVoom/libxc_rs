//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1006/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1006<F: Float>(t11070: F, t6013: F, t10993: F, t2970: F, t6027: F, t10979: F, t133: F, t793: F, t11028: F, t6048: F, t11053: F, t1138: F, t290: F, t2969: F, t2984: F, t3680: F, t3686: F, t3689: F, t6009: F, t6026: F, t6047: F, t7871: F, t7879: F, t791: F, t9695: F, t9707: F) -> (F, F, F, F, F, F, F, F) {
    let t11071 = t11070 * t6013;
    let t11076 = t2970 * t10993;
    let t11079 = t11070 * t6027;
    let t11088 = t10979 * t133;
    let t11089 = t11088 * t793;
    let t11092 = t2970 * t11028;
    let t11095 = t11070 * t6048;
    let t11100 = F::new(0.39512695097613069591e1) * t6009 * t11071 + F::new(0.39512695097613069591e1) * t7871 * t3680 + F::new(0.39512695097613069591e1) * t2969 * t11076 - F::new(0.39512695097613069591e1) * t6026 * t11079 + F::new(0.19756347548806534796e1) * t9707 * t1138 + F::new(0.19756347548806534796e1) * t2984 * t3686 - F::new(0.19756347548806534796e1) * t7879 * t3689 + F::new(0.65854491829355115987e0) * t791 * t11089 - F::new(0.19756347548806534796e1) * t9695 * t11092 + F::new(0.65854491829355115987e0) * t6047 * t11095 + F::new(0.65854491829355115987e0) * t290 * t11053;
    (t11071, t11076, t11079, t11088, t11089, t11092, t11095, t11100)
}
