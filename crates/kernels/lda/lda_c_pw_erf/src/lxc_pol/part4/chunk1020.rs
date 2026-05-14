//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1020/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1020<F: Float>(t12695: F, t1325: F, t4830: F, t2171: F, t3803: F, t5285: F, t571: F, t9678: F, t10654: F, t1318: F, t2034: F, t3854: F, t4684: F, t4624: F, t519: F, t5237: F) -> (F, F, F, F, F, F) {
    let t13309 = t1325 * t12695 * t4830;
    let t13318 = t2171 * t3803;
    let t13351 = t571 * t9678 * t5285;
    let t13358 = t1318 * t10654 * t2034;
    let t13366 = t571 * t3854 * t4684;
    let t13375 = t519 * t5237 * t4624;
    (t13309, t13318, t13351, t13358, t13366, t13375)
}
