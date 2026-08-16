//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 537/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk537<F: Float>(t749: F, t754: F, t97: F, t1786: F, t27: F, t321: F, t106: F, t32: F, t1179: F, t295: F, t315: F, t52: F, t934: F) -> (F, F, F, F, F) {
    let t2760 = t749 * t754 * t97;
    let t2764 = t321 * t1786 * t27;
    let t2765 = t106 * t32;
    let t2767 = t2765 * t1179 * t295;
    let t2771 = t934 * t315 * t52;
    (t2760, t2764, t2765, t2767, t2771)
}
