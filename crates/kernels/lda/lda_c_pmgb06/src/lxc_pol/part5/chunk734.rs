//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 734/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk734<F: Float>(t176: F, t6831: F, t166: F, t2583: F, t435: F, t132: F, t2563: F, t490: F, t1933: F, t831: F, t2554: F, t489: F) -> (F, F, F, F, F, F, F) {
    let t6832 = t6831 * t176;
    let t6833 = t166 * t6832;
    let t6836 = t435 * t2583;
    let t6837 = t132 * t6836;
    let t6839 = t2563 * t490;
    let t6841 = t831 * t1933;
    let t6843 = t489 * t2554;
    (t6832, t6833, t6836, t6837, t6839, t6841, t6843)
}
