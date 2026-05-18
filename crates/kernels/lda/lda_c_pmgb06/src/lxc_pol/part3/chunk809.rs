//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 809/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk809<F: Float>(t5117: F, t5122: F, t5124: F, t5126: F, t5128: F, t5129: F, t5130: F, t5131: F, t5136: F, t5142: F, t5143: F, t5144: F, t5145: F, t5146: F, t5147: F, t5148: F) -> F {
    let t5661 = -t5117 + t5122 + t5124 - t5126 - t5128 - t5129 + t5130 - t5131 - t5136 - t5142 + t5143 + t5144 + t5145 + t5146 + t5147 - t5148;
    t5661
}
