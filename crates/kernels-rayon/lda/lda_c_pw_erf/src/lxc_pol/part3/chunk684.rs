//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 684/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk684(t1619: f64, t225: f64, t10: f64, t602: f64, t1634: f64, t245: f64, t1638: f64, t1627: f64, t1631: f64, t156: f64, t603: f64, t635: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4195 = t225 * t1619;
    let t4196 = t10 * t4195;
    let t4198 = 0.3246312408709453_f64 * t602 * t4196;
    let t4199 = t245 * t1634;
    let t4201 = 0.03354522822333102_f64 * t1638 * t4199;
    let t4202 = t1631 * t1627;
    let t4204 = t156 * t1634;
    let t4206 = 0.21642082724729686_f64 * t602 * t4204;
    let t4207 = t635 * t603;
    (t4195, t4196, t4198, t4199, t4201, t4202, t4204, t4206, t4207)
}
