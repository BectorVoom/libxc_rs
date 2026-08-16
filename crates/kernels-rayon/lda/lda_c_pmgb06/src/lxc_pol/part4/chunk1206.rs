//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1206/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1206(t1423: f64, t6361: f64, t6365: f64, t5211: f64, t6372: f64, t2497: f64, t3226: f64, t2501: f64, t3220: f64, t1972: f64, t5494: f64, t13933: f64, t439: f64, t5272: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15891 = t1423 * t6361;
    let t15892 = 8.0_f64 / 135.0_f64 * t15891;
    let t15893 = t1423 * t6365;
    let t15894 = 8.0_f64 / 135.0_f64 * t15893;
    let t15895 = t5211 * t6372;
    let t15896 = 4.0_f64 / 27.0_f64 * t15895;
    let t15897 = t3226 * t2497;
    let t15898 = 8.0_f64 / 135.0_f64 * t15897;
    let t15899 = t3220 * t2501;
    let t15900 = 8.0_f64 / 135.0_f64 * t15899;
    let t15902 = 4.0_f64 / 45.0_f64 * t1972 * t5494;
    let t15905 = 2.0_f64 / 27.0_f64 * t439 * t13933 * t5272;
    (t15892, t15894, t15896, t15898, t15900, t15902, t15905)
}
