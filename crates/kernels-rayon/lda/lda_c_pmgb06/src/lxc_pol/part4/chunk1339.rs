//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1339/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1339(t13515: f64, t1438: f64, t2106: f64, t5083: f64, t5086: f64, t5108: f64, t851: f64, t1381: f64, t5068: f64, t12537: f64, t13304: f64, t17070: f64) -> (f64, f64, f64, f64) {
    let t17593 = 8.0_f64 / 45.0_f64 * t13515;
    let t17597 = 4.0_f64 / 27.0_f64 * t5083 * t2106 * t1438 * t5086;
    let t17598 = t5108 * t851;
    let t17601 = 8.0_f64 / 45.0_f64 * t5068 * t17598 * t1381;
    let t17604 = 16.0_f64 / 9.0_f64 * t12537 * t13304 * t17070;
    (t17593, t17597, t17601, t17604)
}
