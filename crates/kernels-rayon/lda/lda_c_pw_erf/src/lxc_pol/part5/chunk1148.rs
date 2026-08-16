//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1148/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1148(t17645: f64, t2001: f64, t3974: f64, t16624: f64, t16626: f64, t16633: f64, t16648: f64, t16650: f64, t16652: f64, t16702: f64, t16709: f64, t34: f64, t6335: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t21128 = 16.0_f64 / 15.0_f64 * t3974 * t17645 * t2001;
    let t21129 = 32.0_f64 / 45.0_f64 * t16624;
    let t21130 = 32.0_f64 / 45.0_f64 * t16626;
    let t21131 = 16.0_f64 / 27.0_f64 * t16633;
    let t21132 = 32.0_f64 / 45.0_f64 * t16648;
    let t21133 = 64.0_f64 / 45.0_f64 * t16650;
    let t21134 = 32.0_f64 / 27.0_f64 * t16652;
    let t21135 = 16.0_f64 / 45.0_f64 * t16702;
    let t21136 = 16.0_f64 / 45.0_f64 * t16709;
    let t21137 = t6335 * t34;
    (t21128, t21129, t21130, t21131, t21132, t21133, t21134, t21135, t21136, t21137)
}
