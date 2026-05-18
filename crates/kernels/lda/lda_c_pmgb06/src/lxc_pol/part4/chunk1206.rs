//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1206/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1206<F: Float>(t1423: F, t6361: F, t6365: F, t5211: F, t6372: F, t2497: F, t3226: F, t2501: F, t3220: F, t1972: F, t5494: F, t13933: F, t439: F, t5272: F) -> (F, F, F, F, F, F, F) {
    let t15891 = t1423 * t6361;
    let t15892 = F::new(8.0) / F::new(135.0) * t15891;
    let t15893 = t1423 * t6365;
    let t15894 = F::new(8.0) / F::new(135.0) * t15893;
    let t15895 = t5211 * t6372;
    let t15896 = F::new(4.0) / F::new(27.0) * t15895;
    let t15897 = t3226 * t2497;
    let t15898 = F::new(8.0) / F::new(135.0) * t15897;
    let t15899 = t3220 * t2501;
    let t15900 = F::new(8.0) / F::new(135.0) * t15899;
    let t15902 = F::new(4.0) / F::new(45.0) * t1972 * t5494;
    let t15905 = F::new(2.0) / F::new(27.0) * t439 * t13933 * t5272;
    (t15892, t15894, t15896, t15898, t15900, t15902, t15905)
}
