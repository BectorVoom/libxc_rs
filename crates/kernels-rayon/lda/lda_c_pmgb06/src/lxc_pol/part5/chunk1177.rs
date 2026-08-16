//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1177/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1177(t517: f64, t7616: f64, t161: f64, t489: f64, t7858: f64, t166: f64, t17919: f64, t17921: f64, t17926: f64, t17931: f64, t17935: f64, t17938: f64, t17960: f64, t2088: f64, t21117: f64, t21139: f64, t21184: f64, t21218: f64, t518: f64, t529: f64, t6230: f64, t6736: f64, t802: f64) -> f64 {
    let t21230 = t7616 * t517;
    let t21237 = t161 * t489 * t7858;
    let t21240 = 2.0_f64 / 15.0_f64 * t17919 + 2.0_f64 / 15.0_f64 * t17921 - t161 * t166 * t6230 * t2088 / 10.0_f64 - t161 * t166 * t518 * (t21117 + t21139 + t21184 + t21218) / 30.0_f64 - t802 * t6736 / 10.0_f64 + t17926 / 45.0_f64 + 2.0_f64 / 45.0_f64 * t17931 + 2.0_f64 / 45.0_f64 * t17935 - t161 * t166 * t21230 * t529 / 30.0_f64 - 2.0_f64 / 15.0_f64 * t17938 - t21237 / 45.0_f64 - t17960 / 15.0_f64;
    t21240
}
