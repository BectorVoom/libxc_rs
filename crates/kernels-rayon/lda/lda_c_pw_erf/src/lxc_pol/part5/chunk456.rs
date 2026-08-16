//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 456/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk456(t2166: f64, t542: f64, t1440: f64, t1325: f64, t518: f64, t794: f64) -> (f64, f64, f64, f64) {
    let t2167 = t2166 * t542;
    let t2168 = t1440 * t2167;
    let t2170 = 4.0_f64 / 15.0_f64 * t1325 * t2168;
    let t2171 = t794 * t518;
    (t2167, t2168, t2170, t2171)
}
