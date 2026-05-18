//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1398/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1398<F: Float>(t16159: F, t16162: F, t16167: F, t16171: F, t16172: F, t16174: F, t16176: F, t16179: F, t16182: F, t16185: F, t16187: F, t16189: F, t16190: F, t16192: F, t16195: F) -> F {
    let t18209 = -t16159 - t16162 + t16167 - t16171 + t16172 + t16174 + t16176 - t16179 - t16182 - t16185 + t16187 - t16189 - t16190 - t16192 - t16195;
    t18209
}
