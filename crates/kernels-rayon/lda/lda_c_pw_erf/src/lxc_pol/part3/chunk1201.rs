//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1201/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1201(t14131: f64, t14152: f64, t184: f64, t203: f64, t221: f64, t2954: f64, t3518: f64, t519: f64, t806: f64, t9700: f64, t4753: f64, t5226: f64) -> (f64, f64, f64) {
    let t14157 = 2.0_f64 / 15.0_f64 * t203 * (t14131 + t14152) * t184 * t221;
    let t14162 = 32.0_f64 / 81.0_f64 * t519 * t9700 * t806 * t3518 * t2954;
    let t14164 = 8.0_f64 / 15.0_f64 * t4753 * t5226;
    (t14157, t14162, t14164)
}
