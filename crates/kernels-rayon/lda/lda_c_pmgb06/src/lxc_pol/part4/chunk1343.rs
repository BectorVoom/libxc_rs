//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1343/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1343(t5442: f64, t6268: f64, t1594: f64, t2574: f64, t2864: f64, t439: f64, t15345: f64, t1897: f64, t486: f64, t6851: f64, t13686: f64, t1887: f64, t2108: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17643 = 8.0_f64 / 45.0_f64 * t6268 * t5442;
    let t17647 = 4.0_f64 / 45.0_f64 * t439 * t2864 * t2574 * t1594;
    let t17650 = 8.0_f64 / 15.0_f64 * t439 * t1897 * t15345;
    let t17651 = t486 * t6851;
    let t17652 = 4.0_f64 / 45.0_f64 * t17651;
    let t17653 = 4.0_f64 / 45.0_f64 * t13686;
    let t17655 = 2.0_f64 / 15.0_f64 * t1887 * t2108;
    (t17643, t17647, t17650, t17652, t17653, t17655)
}
