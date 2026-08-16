//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1367/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1367(t5041: f64, t802: f64, t4975: f64, t486: f64, t6232: f64, t161: f64, t1639: f64, t166: f64, t6904: f64, t132: f64, t435: f64, t6226: f64) -> (f64, f64, f64, f64, f64) {
    let t17950 = t802 * t5041 / 15.0_f64;
    let t17952 = 2.0_f64 / 15.0_f64 * t802 * t4975;
    let t17954 = t486 * t6232 / 15.0_f64;
    let t17958 = t161 * t166 * t1639 * t6904 / 15.0_f64;
    let t17960 = t132 * t435 * t6226;
    (t17950, t17952, t17954, t17958, t17960)
}
