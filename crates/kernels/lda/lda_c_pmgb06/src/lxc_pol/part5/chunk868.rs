//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 868/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk868<F: Float>(t2639: F, t955: F, t2645: F, t405: F, t6152: F, t4913: F, t6156: F, t1423: F, t6551: F, t6376: F, t6379: F, t6472: F, t5211: F, t6382: F, t436: F, t6705: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15663 = t955 * t2639;
    let t15671 = t955 * t2645;
    let t15675 = t405 * t6152;
    let t15677 = t4913 * t6156;
    let t15739 = t1423 * t6551;
    let t15764 = t1423 * t6376;
    let t15770 = t1423 * t6379;
    let t15772 = t1423 * t6472;
    let t15774 = t5211 * t6382;
    let t15793 = t6705 * t436;
    (t15663, t15671, t15675, t15677, t15739, t15764, t15770, t15772, t15774, t15793)
}
