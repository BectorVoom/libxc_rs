//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1034/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1034(t1147: f64, t117: f64, t123: f64, t550: f64, t1366: f64, t3312: f64, t3319: f64, t3333: f64, t3325: f64, t184: f64, t186: f64, t247: f64) -> (f64, f64, f64, f64, f64) {
    let t10670 = t123 * t1147 * t550 * t117;
    let t10679 = t3312 * t1366;
    let t10681 = t3319 * t3333;
    let t10684 = 0.04472697096444135_f64 * t3325 * t3333;
    let t10687 = 0.004413481481481482_f64 * t184 * t247 * t186;
    (t10670, t10679, t10681, t10684, t10687)
}
