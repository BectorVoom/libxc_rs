//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 696/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk696<F: Float>(t44: F, t6340: F, t6352: F, t2519: F, t607: F, t4777: F, t2500: F, t2948: F, t439: F, t2064: F, t809: F, t1385: F) -> (F, F, F, F, F, F, F) {
    let t6355 = (t6340 / F::new(2.0) + t6352 / F::new(2.0)) * t44;
    let t6358 = t2519 * t607;
    let t6360 = F::new(4.0) / F::new(405.0) * t4777;
    let t6361 = t2948 * t2500;
    let t6363 = F::new(2.0) / F::new(45.0) * t439 * t6361;
    let t6364 = t809 * t2064;
    let t6365 = t1385 * t6364;
    (t6355, t6358, t6360, t6361, t6363, t6364, t6365)
}
