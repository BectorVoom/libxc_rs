//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 683/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk683<F: Float>(t2164: F, t395: F, t4252: F, t4254: F, t4257: F, t4261: F, t4264: F, t4267: F, t4283: F, t4284: F, t4286: F, t4472: F, t4575: F, t81: F, t1454: F, t1988: F) -> (F, F, F) {
    let t4579 = 0.2133002709687175 * t395 * t2164;
    let t4580 = 0.31995040645307626 * t4472 - 0.10665013548435875 * t4286 + 0.6399008129061525 * t4284 + 0.053059442957798957 * t4261 + 0.10611888591559791 * t4264 + 0.053059442957798957 * t4267 - 0.28298369577492777 * t4254 - 0.28298369577492777 * t4257 - t4283 + t4252 + 0.05332506774217938 * t81 * t4575 - t4579;
    let t4585 = t1988 * t1454;
    (t4579, t4580, t4585)
}
