//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 638/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk638(t3773: f64, t494: f64, t1440: f64, t1325: f64, t1278: f64, t1390: f64, t542: f64, t519: f64, t155: f64, t521: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3774 = t3773 * t494;
    let t3775 = t1440 * t3774;
    let t3777 = 4.0_f64 / 5.0_f64 * t1325 * t3775;
    let t3779 = t1390 * t1278 * t542;
    let t3780 = t1440 * t3779;
    let t3782 = 4.0_f64 / 5.0_f64 * t519 * t3780;
    let t3783 = t155 * t521;
    (t3774, t3775, t3777, t3779, t3780, t3782, t3783)
}
