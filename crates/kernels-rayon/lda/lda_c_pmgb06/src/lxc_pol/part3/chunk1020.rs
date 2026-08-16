//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1020/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1020(t2002: f64, t2971: f64, t3303: f64, t10255: f64, t153: f64, t1859: f64, t439: f64, t4659: f64, t5253: f64, t10247: f64, t4645: f64, t2010: f64, t4655: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12129 = 2.0_f64 / 15.0_f64 * t2002 * t2971;
    let t12131 = t2002 * t3303 / 9.0_f64;
    let t12135 = t439 * t10255 * t153 * t1859 / 9.0_f64;
    let t12138 = t439 * t5253 * t4659 / 9.0_f64;
    let t12139 = t10247 * t153;
    let t12142 = 8.0_f64 / 27.0_f64 * t439 * t12139 * t4645;
    let t12145 = 4.0_f64 / 9.0_f64 * t2010 * t5253 * t4655;
    (t12129, t12131, t12135, t12138, t12142, t12145)
}
