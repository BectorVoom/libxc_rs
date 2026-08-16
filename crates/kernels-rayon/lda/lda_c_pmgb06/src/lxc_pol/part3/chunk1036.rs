//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1036/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk1036(t5375: f64, t591: f64, t4111: f64, t5378: f64, t5382: f64, t5386: f64, t5391: f64, t1542: f64, t1887: f64, t138: f64, t4676: f64, t9175: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12304 = t5375 * t591;
    let t12306 = t5378 * t4111;
    let t12307 = (2e-21_f64 as f64) * t12306;
    let t12308 = t5382 * t591;
    let t12310 = t5386 * t591;
    let t12311 = 4.0_f64 / 3.0_f64 * t12310;
    let t12312 = t5391 * t4111;
    let t12313 = (2e-21_f64 as f64) * t12312;
    let t12315 = t1887 * t1542 / 10.0_f64;
    let t12325 = t138 * t9175 * t4676;
    (t12304, t12307, t12308, t12311, t12313, t12315, t12325)
}
