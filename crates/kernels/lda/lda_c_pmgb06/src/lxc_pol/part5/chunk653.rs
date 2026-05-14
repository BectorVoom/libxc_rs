//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 653/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk653<F: Float>(t12: F, t15: F, t2389: F, t1072: F, t1949: F, t337: F, t5974: F, t598: F, t6341: F, t44: F, t6340: F, t2519: F, t607: F, t4777: F, t2500: F, t2948: F, t439: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t13 = t12 <= zeta_threshold;
    let t6346 = t15 * t2389;
    let t6352 = piecewise3(t13, 0.0, 80.0 / 27.0 * t6341 * t337 - 160.0 / 9.0 * t1949 * t1072 + 40.0 / 9.0 * t6346 * t337 + 8.0 / 3.0 * t598 * t5974);
    let t6355 = (t6340 / 2.0 + t6352 / 2.0) * t44;
    let t6358 = t2519 * t607;
    let t6360 = 4.0 / 405.0 * t4777;
    let t6361 = t2948 * t2500;
    let t6363 = 2.0 / 45.0 * t439 * t6361;
    (t6355, t6358, t6360, t6361, t6363)
}
