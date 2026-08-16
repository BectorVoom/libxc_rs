//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1210/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1210(t1318: f64, t1319: f64, t3563: f64, t816: f64, t1287: f64, t1954: f64, t4758: f64, t3787: f64, t4937: f64, t519: f64, t1440: f64, t3677: f64, t806: f64, t9223: f64) -> (f64, f64, f64, f64) {
    let t14271 = 8.0_f64 / 45.0_f64 * t1318 * t1319 * t816 * t3563;
    let t14275 = 16.0_f64 / 15.0_f64 * t1318 * t4758 * t1954 * t1287;
    let t14277 = t519 * t3787 * t4937;
    let t14278 = 8.0_f64 / 5.0_f64 * t14277;
    let t14283 = 16.0_f64 / 5.0_f64 * t519 * t1440 * t9223 * t806 * t3677;
    (t14271, t14275, t14278, t14283)
}
