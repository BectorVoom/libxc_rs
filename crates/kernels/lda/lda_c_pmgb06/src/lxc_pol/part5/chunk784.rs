//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 784/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk784<F: Float>(t1092: F, t1105: F, t2799: F, t687: F, t3767: F, t638: F, t1101: F, t643: F, t248: F, t3890: F, t653: F, t1024: F, t3697: F, t634: F, t3963: F, t3969: F) -> (F, F, F, F, F, F, F, F) {
    let t8529 = t1105 * t1092;
    let t8536 = t2799 * t687;
    let t8538 = t638 * t3767;
    let t8543 = t1101 * t1092;
    let t8545 = t643 * t3767;
    let t8548 = t248 * t653 * t3890;
    let t8552 = 8.0 * t1024 * t634 * t3697;
    let t8553 = t3969 * t3963;
    (t8529, t8536, t8538, t8543, t8545, t8548, t8552, t8553)
}
