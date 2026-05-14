//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 839/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk839<F: Float>(t5068: F, t6630: F, t5139: F, t6629: F, t5138: F, t1593: F, t443: F, t760: F, t822: F) -> (F, F, F, F, F) {
    let t6632 = 4.0 / 45.0 * t5068 * t6630;
    let t6633 = t5139 * t6629;
    let t6635 = 2.0 / 27.0 * t5138 * t6633;
    let t6636 = t1593 * t443;
    let t6637 = t760 * t822;
    (t6632, t6633, t6635, t6636, t6637)
}
