//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1047/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1047<F: Float>(t325: F, t7440: F, t352: F, t7408: F, t11: F, t1349: F, t20907: F, t3633: F, t1953: F, t20911: F, t21207: F, t21211: F, t7365: F, t9410: F, t10102: F, t34: F, t6383: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t21792 = t325 * t7440;
    let t21794 = t7408 * t352;
    let t21796 = t11 * t1349 * t21794;
    let t21799 = t11 * t3633 * t20907;
    let t21802 = t1953 * t1349 * t20911;
    let t21805 = t11 * t1349 * t21207;
    let t21808 = t1953 * t1349 * t21211;
    let t21811 = t9410 * t7365 * t352;
    let t21813 = t11 * t10102 * t21811;
    let t21815 = t6383 * t34;
    (t21792, t21794, t21796, t21799, t21802, t21805, t21808, t21811, t21813, t21815)
}
