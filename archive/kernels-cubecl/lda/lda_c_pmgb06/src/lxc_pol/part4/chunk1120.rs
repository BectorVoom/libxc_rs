//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1120/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1120<F: Float>(t1193: F, t1354: F, t14277: F, t1798: F, t740: F, t409: F, t419: F, t421: F, t4463: F, t1186: F, t5617: F, t2329: F, t2837: F) -> (F, F, F, F, F, F) {
    let t14279 = t14277 * t1193 * t1354;
    let t14281 = t740 * t1798;
    let t14283 = t14281 * t1193 * t1354;
    let t14287 = t409 * t4463 * t419 * t421;
    let t14290 = t5617 * t1186 * t421;
    let t14293 = t2329 * t2837 * t421;
    (t14279, t14281, t14283, t14287, t14290, t14293)
}
