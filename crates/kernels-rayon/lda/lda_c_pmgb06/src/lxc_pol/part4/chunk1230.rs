//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1230/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1230(t1420: f64, t6472: f64, t439: f64, t5253: f64, t6146: f64, t15373: f64, t1901: f64, t15378: f64, t5260: f64, t5474: f64, t6268: f64, t1894: f64, t5220: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16201 = 4.0_f64 / 9.0_f64 * t1420 * t6472;
    let t16204 = 4.0_f64 / 9.0_f64 * t439 * t5253 * t6146;
    let t16207 = 2.0_f64 / 9.0_f64 * t439 * t1901 * t15373;
    let t16210 = 32.0_f64 / 27.0_f64 * t439 * t5260 * t15378;
    let t16212 = 8.0_f64 / 27.0_f64 * t6268 * t5474;
    let t16213 = t5220 * t1894;
    (t16201, t16204, t16207, t16210, t16212, t16213)
}
