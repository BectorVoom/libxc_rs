//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1116/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1116<F: Float>(t4906: F, t4913: F, t1464: F, t524: F, t2911: F, t3357: F, t146: F, t4918: F, t9712: F, t1575: F, t2918: F, t13560: F, t2085: F) -> (F, F, F, F, F, F) {
    let t14082 = t4913 * t4906;
    let t14106 = t524 * t1464;
    let t14110 = t3357 * t2911;
    let t14150 = t146 * t9712 * t4918;
    let t14152 = t1575 * t2918;
    let t14162 = t13560 * t2085;
    (t14082, t14106, t14110, t14150, t14152, t14162)
}
