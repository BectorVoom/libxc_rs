//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1143/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1143<F: Float>(t2036: F, t30910: F, t11053: F, t751: F, t11071: F, t11076: F, t11089: F, t11095: F, t1123: F, t1138: F, t18278: F, t18338: F, t2019: F, t2131: F, t22082: F, t26646: F, t26653: F, t287: F, t290: F, t2971: F, t2977: F, t2981: F, t2984: F, t30807: F, t30885: F, t30893: F, t30897: F, t3680: F, t3686: F, t5718: F, t5952: F, t759: F, t7832: F, t7837: F, t7874: F, t794: F, t9277: F, t9314: F, t9662: F, t9670: F, t9671: F, t9692: F, t9695: F, t9700: F, t9703: F, t9707: F) -> (F,) {
    let t30931 = t2036 * t30910;
    let t30947 = t751 * t11053;
    let t30977 = -0.19756347548806534796e1 * t9695 * t9700 - 0.11853808529283920877e2 * t9670 * t7832 * t9314 - 0.65854491829355115987e0 * t30931 * t2981 + 0.19756347548806534796e1 * t9703 * t7832 * t9277 + 0.19756347548806534796e1 * t26646 * t7832 * t1123 * t759 * t287 + 0.65854491829355115987e0 * t2131 * t11089 + 0.11853808529283920877e2 * t5952 * t30893 * t9662 + 0.65854491829355115987e0 * t30947 * t794 + 0.39512695097613069591e1 * t18278 * t11071 + 0.19756347548806534796e1 * t7874 * t3686 - 0.11853808529283920877e2 * t5718 * t30893 * t9671 + 0.39512695097613069591e1 * t7837 * t11076 + 0.19756347548806534796e1 * t9707 * t2977 + 0.39512695097613069592e1 * t2019 * t30897 * t2971 + 0.65854491829355115987e0 * t290 * t30807 + 0.39512695097613069591e1 * t22082 * t3680 + 0.19756347548806534796e1 * t2984 * t9692 + 0.65854491829355115987e0 * t18338 * t11095 - 0.19756347548806534796e1 * t2036 * t30885 * t2981 + 0.19756347548806534796e1 * t26653 * t1138;
    (t30977,)
}
