//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 607/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk607<F: Float>(t1835: F, t495: F, t2065: F, t435: F, t132: F, t2015: F, t432: F, t1423: F, t1908: F, t1898: F, t1902: F, t1447: F, t1925: F, t1894: F, t2022: F, t591: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5312 = t495 * t1835;
    let t5326 = t435 * t2065;
    let t5328 = 2.0 / 45.0 * t132 * t5326;
    let t5330 = 2.0 / 45.0 * t432 * t2015;
    let t5342 = 4.0 / 135.0 * t1423 * t1908;
    let t5349 = 8.0 / 135.0 * t1423 * t1898;
    let t5354 = 4.0 / 81.0 * t1423 * t1902;
    let t5356 = 4.0 / 135.0 * t1447 * t1925;
    let t5363 = 4.0 / 135.0 * t1423 * t1894;
    let t5369 = 4.0 / 9.0 * t2022 * t591;
    (t5312, t5326, t5328, t5330, t5342, t5349, t5354, t5356, t5363, t5369)
}
