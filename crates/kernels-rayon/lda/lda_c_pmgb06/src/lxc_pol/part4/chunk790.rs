//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 790/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk790(t4645: f64, t5260: f64, t439: f64, t1901: f64, t4655: f64, t2010: f64, t1074: f64, t1906: f64, t1385: f64, t1438: f64, t822: f64, t1069: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5261 = t5260 * t4645;
    let t5263 = 8.0_f64 / 81.0_f64 * t439 * t5261;
    let t5264 = t1901 * t4655;
    let t5266 = 4.0_f64 / 27.0_f64 * t2010 * t5264;
    let t5267 = t1906 * t1074;
    let t5268 = t1385 * t5267;
    let t5270 = t439 * t5268 / 45.0_f64;
    let t5271 = t822 * t1438;
    let t5272 = t5271 * t1069;
    (t5261, t5263, t5264, t5266, t5267, t5268, t5270, t5272)
}
