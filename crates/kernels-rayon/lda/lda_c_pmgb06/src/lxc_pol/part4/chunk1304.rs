//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1304/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1304(t350: f64, t6821: f64, t1464: f64, t337: f64, t5974: f64, t1476: f64, t36: f64, t1083: f64, t6764: f64, t1080: f64, t2389: f64, t2911: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17140 = t350 * t6821;
    let t17143 = t1464 * t5974 * t337;
    let t17145 = t36 * t1476 * t17143;
    let t17147 = t6764 * t1083;
    let t17149 = t36 * t1476 * t17147;
    let t17152 = t2911 * t2389 * t1080;
    (t17140, t17143, t17145, t17147, t17149, t17152)
}
