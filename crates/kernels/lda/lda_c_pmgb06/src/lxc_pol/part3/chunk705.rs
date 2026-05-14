//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 705/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk705<F: Float>(t5136: F, t5142: F, t5143: F, t5144: F, t5145: F, t5146: F, t5147: F, t5148: F, t5149: F, t5150: F, t5151: F, t5152: F, t5153: F, t5154: F, t5155: F, t3227: F) -> (F, F) {
    let t5156 = -t5136 - t5142 + t5143 + t5144 + t5145 + t5146 + t5147 - t5148 + t5149 + t5150 + t5151 - t5152 - t5153 + t5154 - t5155;
    let t5157 = 4.0 / 135.0 * t3227;
    (t5156, t5157)
}
