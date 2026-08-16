//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 846/1239 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk846(t1272: f64, t4913: f64, t1239: f64, t1234: f64, t315: f64, t934: f64, t1238: f64, t64: f64, t97: f64, t342: f64, t740: f64, t3576: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8293 = 2.9018074074074076_f64 * t1272 * t4913;
    let t8295 = 5.773876543209877_f64 * t1239 * t4913;
    let t8299 = t934 * t315 * t1234;
    let t8300 = t1238 * t64 * t97 * t8299;
    let t8305 = t934 * t740 * t342;
    let t8306 = t3576 * t8305;
    (t8293, t8295, t8299, t8300, t8305, t8306)
}
