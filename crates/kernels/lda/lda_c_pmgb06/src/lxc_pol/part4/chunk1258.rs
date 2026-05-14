//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1258/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1258<F: Float>(t10903: F, t10905: F, t1167: F, t123: F, t14663: F, t14666: F, t14669: F, t14672: F, t18437: F, t18954: F, t18969: F, t2422: F, t305: F, t6939: F, t726: F, t395: F, t6104: F) -> (F, F) {
    let t18973 = -0.2133002709687175 * t14663 + 0.31995040645307626 * t18954 - 0.031835665774679375 * t123 * t305 * t18437 - 0.031835665774679375 * t123 * t1167 * t2422 + 1.0376068845080684 * t14666 + 1.0376068845080684 * t14669 + 0.10611888591559791 * t14672 - 0.06367133154935875 * t123 * t726 * t6939 + 0.10611888591559791 * t18969 + 0.31995040645307626 * t10903 - 2.55960325162461 * t10905;
    let t18979 = t395 * t6104;
    (t18973, t18979)
}
