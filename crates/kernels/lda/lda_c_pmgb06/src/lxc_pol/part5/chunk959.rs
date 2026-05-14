//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 959/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk959<F: Float>(t5: F, t1420: F, t7667: F, t1426: F, t439: F, t7666: F, t1444: F, t7671: F, t1962: F, t6244: F, t7577: F, t12092: F, t2484: F, t7581: F, t7580: F, t9596: F, t1072: F, t19870: F, t2381: F, t332: F, t5961: F, t7290: F, t760: F, zeta_threshold: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6 = t5 <= zeta_threshold;
    let t20071 = 2.0 / 15.0 * t1420 * t7667;
    let t20074 = 2.0 / 15.0 * t439 * t1426 * t7666;
    let t20076 = 2.0 / 15.0 * t1444 * t7671;
    let t20079 = t439 * t1962 * t6244 / 15.0;
    let t20081 = t1420 * t7577 / 9.0;
    let t20084 = t439 * t12092 * t2484 / 9.0;
    let t20086 = t1420 * t7581 / 45.0;
    let t20089 = t439 * t1426 * t7580 / 45.0;
    let t20090 = 4.0 / 405.0 * t9596;
    let t20100 = piecewise3(t6, 0.0, 12.0 * t1072 * t2381 + 2.0 * t19870 * t5 + 2.0 * t332 * t7290 + 6.0 * t5961 * t760);
    (t20071, t20074, t20076, t20079, t20081, t20084, t20086, t20089, t20090, t20100)
}
