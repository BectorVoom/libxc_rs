//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1332/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1332(t486: f64, t6731: f64, t1499: f64, t2654: f64, t6461: f64, t1969: f64, t5187: f64, t17482: f64, t17487: f64, t17490: f64, t17493: f64, t17496: f64, t17497: f64, t17499: f64, t17503: f64, t17505: f64, t17507: f64, t17509: f64) -> (f64, f64, f64, f64, f64) {
    let t17511 = 2.0_f64 / 15.0_f64 * t486 * t6731;
    let t17513 = t1499 * t2654 / 15.0_f64;
    let t17515 = 2.0_f64 / 15.0_f64 * t486 * t6461;
    let t17517 = 4.0_f64 / 15.0_f64 * t5187 * t1969;
    let t17518 = -t17482 - t17487 - t17490 - t17493 - t17496 + t17497 - t17499 - t17503 - t17505 - t17507 - t17509 - t17511 - t17513 - t17515 + t17517;
    (t17511, t17513, t17515, t17517, t17518)
}
