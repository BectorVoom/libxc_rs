//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1012/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1012(t1152: f64, t123: f64, t2422: f64, t6939: f64, t722: f64, t2753: f64, t754: f64, t936: f64, t97: f64, t1786: f64, t1789: f64, t409: f64) -> (f64, f64, f64, f64) {
    let t19020 = t123 * t1152 * t2422;
    let t19031 = t123 * t722 * t6939;
    let t19055 = t2753 * t754 * t97 * t936;
    let t19063 = t409 * t2753 * t1786 * t1789;
    (t19020, t19031, t19055, t19063)
}
