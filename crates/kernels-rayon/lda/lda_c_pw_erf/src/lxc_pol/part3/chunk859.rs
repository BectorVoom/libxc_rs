//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 859/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk859(t1029: f64, t400: f64, t8170: f64, t8171: f64, t1055: f64, t2742: f64, t1059: f64, t2987: f64, t1063: f64, t2694: f64, t296: f64, t905: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8173 = t1029 * t1029;
    let t8174 = 1.0_f64 / t8173;
    let t8177 = 91080.98259910992_f64 * t400 * t8170 * t8171 * t8174;
    let t8178 = t2742 * t1055;
    let t8180 = t1059 * t2987;
    let t8184 = 4.740006021527056_f64 * t2694 * t1063 * t296;
    let t8185 = t905 * t905;
    (t8174, t8177, t8178, t8180, t8184, t8185)
}
