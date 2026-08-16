//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1324/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1324(t2389: f64, t337: f64, t529: f64, t5068: f64, t5069: f64, t5138: f64, t5139: f64, t1414: f64, t2093: f64, t5071: f64, t1420: f64, t6551: f64) -> (f64, f64, f64, f64) {
    let t17404 = t2389 * t529 * t337;
    let t17407 = 4.0_f64 / 45.0_f64 * t5068 * t5069 * t17404;
    let t17410 = 2.0_f64 / 27.0_f64 * t5138 * t5139 * t17404;
    let t17414 = 8.0_f64 / 45.0_f64 * t5068 * t2093 * t1414 * t5071;
    let t17416 = 4.0_f64 / 15.0_f64 * t1420 * t6551;
    (t17407, t17410, t17414, t17416)
}
