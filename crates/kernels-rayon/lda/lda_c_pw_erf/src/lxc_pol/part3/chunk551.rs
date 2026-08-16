//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 551/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk551(t147: f64, t285: f64, t2872: f64, t281: f64, t1191: f64) -> (f64, f64, f64) {
    let t2874 = t147 * t2872 * t285;
    let t2876 = 0.01197423401025461_f64 * t281 * t2874;
    let t2877 = t1191 * t147;
    (t2874, t2876, t2877)
}
