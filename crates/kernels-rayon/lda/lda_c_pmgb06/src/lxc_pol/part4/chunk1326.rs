//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1326/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1326(t1083: f64, t6560: f64, t5138: f64, t5139: f64, t12529: f64, t12530: f64, t16386: f64, t13312: f64, t15264: f64, t337: f64, t12537: f64, t11904: f64, t6562: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17427 = t6560 * t1083;
    let t17430 = 2.0_f64 / 27.0_f64 * t5138 * t5139 * t17427;
    let t17433 = 16.0_f64 / 81.0_f64 * t12529 * t12530 * t16386;
    let t17434 = 16.0_f64 / 135.0_f64 * t13312;
    let t17435 = t15264 * t337;
    let t17438 = 8.0_f64 / 27.0_f64 * t12537 * t5139 * t17435;
    let t17440 = 8.0_f64 / 45.0_f64 * t11904 * t6562;
    (t17427, t17430, t17433, t17434, t17435, t17438, t17440)
}
