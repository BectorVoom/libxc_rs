//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1124/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1124<F: Float>(t12568: F, t1476: F, t1830: F, t350: F, t4881: F, t4886: F, t12584: F, t36: F, t12594: F, t9507: F, t11997: F, t506: F) -> (F, F, F, F, F, F) {
    let t13343 = t1830 * t1476 * t12568;
    let t13345 = t350 * t4881;
    let t13347 = t350 * t4886;
    let t13350 = t36 * t1476 * t12584;
    let t13353 = t36 * t9507 * t12594;
    let t13356 = t1830 * t506 * t11997;
    (t13343, t13345, t13347, t13350, t13353, t13356)
}
