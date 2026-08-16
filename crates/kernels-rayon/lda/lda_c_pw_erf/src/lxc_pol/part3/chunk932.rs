//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 932/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk932(t331: f64, t3620: f64, t3611: f64, t4233: f64, t598: f64, t226: f64, t4606: f64, t5021: f64, t7: f64, t1397: f64, t4073: f64, t1472: f64, t3748: f64) -> (f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t10250 = t331 * t3620;
    let t10252 = t331 * t3611;
    let t10278 = t598 * t4233;
    let t10286 = 4.0_f64 / 3.0_f64 * t226 * (-4.277777777777778_f64 * t4606 + 220.0_f64 / 81.0_f64 * t5021) * pi * t7;
    let t10294 = t4073 * t1397;
    let t10296 = t1472 * t3748;
    (t10250, t10252, t10278, t10286, t10294, t10296)
}
