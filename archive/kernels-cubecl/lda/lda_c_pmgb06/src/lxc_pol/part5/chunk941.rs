//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 941/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk941<F: Float>(t2060: F, t819: F, t8088: F, t99: F, t2061: F, t102: F, t147: F, t3092: F, t3403: F, t1438: F, t472: F, t1618: F, t3098: F) -> (F, F, F, F, F, F, F) {
    let t13558 = t2060 * t819;
    let t13560 = t99 * t8088;
    let t13561 = t13560 * t2061;
    let t13565 = t99 * t102 * t147;
    let t13566 = t3403 * t3092;
    let t13570 = t472 * t1438;
    let t13574 = t1618 * t3098;
    (t13558, t13560, t13561, t13565, t13566, t13570, t13574)
}
