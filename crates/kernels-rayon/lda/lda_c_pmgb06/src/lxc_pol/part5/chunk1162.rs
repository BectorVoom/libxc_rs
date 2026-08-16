//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1162/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1162(t17276: f64, t493: f64, t6508: f64, t1981: f64, t6512: f64, t6751: f64, t1972: f64, t6756: f64, t6761: f64, t6766: f64, t1: f64, t1380: f64, t6781: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20964 = 8.0_f64 / 27.0_f64 * t493 * t17276 * t6508;
    let t20967 = 4.0_f64 / 9.0_f64 * t1981 * t6751 * t6512;
    let t20969 = t1972 * t6756 / 15.0_f64;
    let t20971 = 2.0_f64 / 15.0_f64 * t1972 * t6761;
    let t20973 = t1972 * t6766 / 9.0_f64;
    let t20977 = 2.0_f64 / 15.0_f64 * t1981 * t1380 * t6781 * t1;
    (t20964, t20967, t20969, t20971, t20973, t20977)
}
