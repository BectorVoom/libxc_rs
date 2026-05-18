//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 717/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk717<F: Float>(t285: F, t4515: F, t2160: F, t638: F, t1105: F, t898: F, t1101: F, t1065: F, t897: F, t248: F, t1108: F, t2142: F, t27: F) -> (F, F, F, F, F, F, F, F) {
    let t4516 = t4515 * t285;
    let t4518 = t638 * t2160;
    let t4520 = t1105 * t898;
    let t4522 = t1101 * t898;
    let t4524 = t897 * t1065;
    let t4525 = t248 * t4524;
    let t4527 = t1108 * t898;
    let t4529 = t2142 * t27;
    (t4516, t4518, t4520, t4522, t4524, t4525, t4527, t4529)
}
