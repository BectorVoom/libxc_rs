//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 696/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk696<F: Float>(t1386: F, t5078: F, t5077: F, t1435: F, t5066: F, t5075: F) -> (F, F, F, F) {
    let t5079 = t5078 * t1386;
    let t5081 = 4.0 / 45.0 * t5077 * t5079;
    let t5082 = t5066 * t1435;
    let t5083 = t5075 * t5082;
    (t5079, t5081, t5082, t5083)
}
