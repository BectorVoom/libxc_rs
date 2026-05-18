//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1332/1478 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1332<F: Float>(t486: F, t6731: F, t1499: F, t2654: F, t6461: F, t1969: F, t5187: F, t17482: F, t17487: F, t17490: F, t17493: F, t17496: F, t17497: F, t17499: F, t17503: F, t17505: F, t17507: F, t17509: F) -> (F, F, F, F, F) {
    let t17511 = F::new(2.0) / F::new(15.0) * t486 * t6731;
    let t17513 = t1499 * t2654 / F::new(15.0);
    let t17515 = F::new(2.0) / F::new(15.0) * t486 * t6461;
    let t17517 = F::new(4.0) / F::new(15.0) * t5187 * t1969;
    let t17518 = -t17482 - t17487 - t17490 - t17493 - t17496 + t17497 - t17499 - t17503 - t17505 - t17507 - t17509 - t17511 - t17513 - t17515 + t17517;
    (t17511, t17513, t17515, t17517, t17518)
}
