//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 612/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk612(t2954: f64, t3518: f64, t3516: f64, t2961: f64, t504: f64, t538: f64, t503: f64, t11: f64, t506: f64, t925: f64, t1257: f64, t325: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3519 = t3518 * t2954;
    let t3520 = t3516 * t3519;
    let t3523 = t504 * t2961;
    let t3524 = t538 * t3523;
    let t3527 = t503 * t3523;
    let t3528 = t11 * t3527;
    let t3530 = t925 * t506;
    let t3532 = t325 * t1257;
    (t3519, t3520, t3524, t3527, t3528, t3530, t3532)
}
