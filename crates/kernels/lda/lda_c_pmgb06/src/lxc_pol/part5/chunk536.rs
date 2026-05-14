//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 536/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk536<F: Float>(t1554: F, t512: F, t161: F, t176: F, t2918: F, t153: F, t3098: F, t129: F, t1710: F) -> (F, F, F, F, F) {
    let t3155 = t1554 * t512;
    let t3156 = t161 * t3155;
    let t3172 = t176 * t2918;
    let t3189 = t153 * t3098;
    let t3213 = t129 * t1710;
    (t3155, t3156, t3172, t3189, t3213)
}
