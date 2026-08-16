//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 969/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk969<F: Float>(t130: F, t801: F, t5076: F, t5082: F, t830: F, t5067: F, t5137: F, t5499: F, t6395: F, t1423: F, t6361: F, t6365: F) -> (F, F, F, F, F, F, F) {
    let t15854 = t801 * t130;
    let t15855 = t15854 * t5076;
    let t15858 = t15854 * t5082;
    let t15861 = t830 * t130;
    let t15862 = t15861 * t5067;
    let t15865 = t15861 * t5137;
    let t15887 = t5499 * t6395;
    let t15891 = t1423 * t6361;
    let t15893 = t1423 * t6365;
    (t15855, t15858, t15862, t15865, t15887, t15891, t15893)
}
