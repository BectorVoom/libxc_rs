//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1047/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1047(t281: f64, t285: f64, t477: f64, t6039: f64, t1128: f64, t2363: f64, t142: f64, t6121: f64, t455: f64, t1549: f64, t6097: f64, t169: f64, t242: f64, t299: f64) -> (f64, f64, f64, f64, f64) {
    let t18888 = t281 * t6039 * t477 * t285;
    let t18892 = t281 * t2363 * t1128 * t285;
    let t18900 = t142 * t6121;
    let t18901 = t455 * t18900;
    let t18906 = t1549 * t6097;
    let t18918 = t169 * t299 * t6039 * t242;
    (t18888, t18892, t18901, t18906, t18918)
}
