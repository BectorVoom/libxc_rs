//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1055/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1055(t12546: f64, t12547: f64, t5068: f64, t2956: f64, t5077: f64, t5078: f64, t12514: f64, t441: f64, t5075: f64, t5079: f64, t1083: f64, t4851: f64) -> (f64, f64, f64, f64, f64) {
    let t12550 = 2.0_f64 / 5.0_f64 * t5068 * t12546 * t12547;
    let t12553 = 2.0_f64 / 15.0_f64 * t5077 * t5078 * t2956;
    let t12555 = t5075 * t12514 * t441;
    let t12556 = t12555 * t5079;
    let t12557 = 8.0_f64 / 45.0_f64 * t12556;
    let t12558 = t4851 * t1083;
    (t12550, t12553, t12555, t12557, t12558)
}
