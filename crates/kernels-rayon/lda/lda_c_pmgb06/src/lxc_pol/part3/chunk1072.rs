//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1072/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1072(t1420: f64, t5268: f64, t2948: f64, t439: f64, t5267: f64, t1074: f64, t1385: f64, t5231: f64, t1906: f64, t3115: f64, t5273: f64, t10148: f64, t5272: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12730 = t1420 * t5268 / 15.0_f64;
    let t12733 = t439 * t2948 * t5267 / 15.0_f64;
    let t12737 = t439 * t1385 * t5231 * t1074 / 15.0_f64;
    let t12741 = t439 * t1385 * t1906 * t3115 / 45.0_f64;
    let t12743 = t1420 * t5273 / 9.0_f64;
    let t12746 = t439 * t10148 * t5272 / 9.0_f64;
    (t12730, t12733, t12737, t12741, t12743, t12746)
}
