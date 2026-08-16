//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 789/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk789(t2034: f64, t3854: f64, t1318: f64, t1403: f64, t816: f64, t3867: f64, t571: f64, t1390: f64, t1440: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5282 = t3854 * t2034;
    let t5284 = 32.0_f64 / 135.0_f64 * t1318 * t5282;
    let t5285 = t816 * t1403;
    let t5286 = t3867 * t5285;
    let t5288 = 8.0_f64 / 45.0_f64 * t571 * t5286;
    let t5289 = t1440 * t1390;
    (t5282, t5284, t5285, t5286, t5288, t5289)
}
