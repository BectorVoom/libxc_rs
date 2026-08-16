//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 678/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk678(t1138: f64, t1597: f64, t2881: f64, t2910: f64, t482: f64, t485: f64, t1098: f64, t2916: f64, t1186: f64, t1124: f64, t465: f64, t483: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4156 = t2881 * t1138 * t1597;
    let t4160 = 0.005926167098672845_f64 * t482 * t2910 * t485;
    let t4163 = 0.0014862827083471494_f64 * t1098 * t2916 * t1597;
    let t4165 = 0.025899545097903542_f64 * t1186 * t485;
    let t4166 = t1124 * t465;
    let t4168 = t4166 * t483 * t485;
    (t4156, t4160, t4163, t4165, t4166, t4168)
}
