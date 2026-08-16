//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 927/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk927(t12239: f64, t3450: f64, t831: f64, t1592: f64, t1872: f64, t5375: f64, t591: f64, t4111: f64, t5378: f64, t5386: f64, t5391: f64, t1869: f64, t8337: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12240 = t12239 / 45.0_f64;
    let t12245 = t831 * t3450;
    let t12246 = t12245 / 45.0_f64;
    let t12252 = t1872 * t1592;
    let t12304 = t5375 * t591;
    let t12306 = t5378 * t4111;
    let t12310 = t5386 * t591;
    let t12311 = 4.0_f64 / 3.0_f64 * t12310;
    let t12312 = t5391 * t4111;
    let t12313 = (2e-21_f64 as f64) * t12312;
    let t12329 = t8337 * t1869;
    (t12240, t12246, t12252, t12304, t12306, t12311, t12313, t12329)
}
