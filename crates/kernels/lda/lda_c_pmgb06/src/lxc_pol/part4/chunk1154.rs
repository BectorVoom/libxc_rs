//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1154/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1154<F: Float>(t17188: F, t1919: F, t493: F, t1444: F, t6766: F, t5463: F, t6765: F, t17143: F, t17147: F, t17152: F, t5470: F, t1464: F, t2093: F, t5071: F, t5138: F, t2389: F, t337: F, t529: F) -> (F, F, F, F, F, F, F, F) {
    let t17384 = 2.0 / 9.0 * t493 * t1919 * t17188;
    let t17386 = 2.0 / 27.0 * t1444 * t6766;
    let t17389 = 2.0 / 27.0 * t493 * t5463 * t6765;
    let t17392 = 2.0 / 27.0 * t493 * t1919 * t17143;
    let t17395 = t493 * t1919 * t17147 / 27.0;
    let t17398 = 8.0 / 81.0 * t493 * t5470 * t17152;
    let t17402 = 4.0 / 27.0 * t5138 * t2093 * t1464 * t5071;
    let t17404 = t2389 * t529 * t337;
    (t17384, t17386, t17389, t17392, t17395, t17398, t17402, t17404)
}
