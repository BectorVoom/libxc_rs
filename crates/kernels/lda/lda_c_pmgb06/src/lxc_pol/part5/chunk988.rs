//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 988/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk988<F: Float>(t405: F, t6891: F, t4913: F, t6894: F, t6897: F, t6900: F, t13483: F, t176: F, t1447: F, t6756: F, t6761: F, t6766: F) -> (F, F, F, F, F, F, F, F) {
    let t17215 = t405 * t6891;
    let t17217 = t4913 * t6894;
    let t17222 = t405 * t6897;
    let t17224 = t405 * t6900;
    let t17276 = t13483 * t176;
    let t17283 = t1447 * t6756;
    let t17285 = t1447 * t6761;
    let t17287 = t1447 * t6766;
    (t17215, t17217, t17222, t17224, t17276, t17283, t17285, t17287)
}
