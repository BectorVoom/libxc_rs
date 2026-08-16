//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 850/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk850(t1272: f64, t4913: f64, t1239: f64, t342: f64, t740: f64, t934: f64, t3576: f64, t28: f64, t3: f64, t37: f64, t27: f64, t4238: f64, t55: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8293 = 2.9018074074074076_f64 * t1272 * t4913;
    let t8295 = 5.773876543209877_f64 * t1239 * t4913;
    let t8305 = t934 * t740 * t342;
    let t8306 = t3576 * t8305;
    let t8333 = 1.0_f64 / t37 / t28 / t3 / 48.0_f64;
    let t8337 = t4238 * t27 * t55;
    (t8293, t8295, t8305, t8306, t8333, t8337)
}
