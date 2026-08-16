//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 626/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk626(t188: f64, t3675: f64, t1392: f64, t542: f64, t186: f64, t185: f64, t1217: f64, t665: f64, t1231: f64, t668: f64, t348: f64, t92: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3676 = t188 * t3675;
    let t3677 = t1392 * t542;
    let t3678 = t3676 * t3677;
    let t3679 = t186 * t3678;
    let t3681 = 4.0_f64 / 5.0_f64 * t185 * t3679;
    let t3682 = t665 * t1217;
    let t3684 = t1231 * t668;
    let t3688 = t92 * t348;
    (t3677, t3678, t3679, t3681, t3682, t3684, t3688)
}
