//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 954/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk954(t2676: f64, t27: f64, t545: f64, t3007: f64, t3026: f64, t3028: f64, t5104: f64, t5107: f64, t5114: f64, t6430: f64, t6433: f64, t6434: f64, t6440: f64, t6445: f64, t6447: f64, t6451: f64, t6453: f64) -> (f64, f64) {
    let t7193 = t2676 * t27;
    let t7194 = t7193 * t545;
    let t7196 = -t6430 - t6433 - t6434 + t3007 + t6440 + t6445 + t6447 + t6451 - t6453 + t3026 + 4.0_f64 / 3.0_f64 * t3028 + 0.10821041362364843_f64 * t7194 - t5104 - t5107 - t5114;
    (t7193, t7196)
}
