//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 885/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk885(t155: f64, t174: f64, t3135: f64, t3137: f64, t2745: f64, t3123: f64, t902: f64, t906: f64, t13: f64, t8185: f64, t3128: f64, t907: f64) -> (f64, f64, f64, f64) {
    let t8397 = 6.873371715287382_f64 * t174 * t155 * t3135 * t3137;
    let t8400 = 0.4274_f64 * t174 * t2745 * t3123;
    let t8407 = t902 * t902;
    let t8410 = t906 * t906;
    let t8414 = 24954.97798673547_f64 * t13 / t8407 * t8185 / t8410;
    let t8417 = 578.9456755974397_f64 * t3128 * t8185 * t907;
    (t8397, t8400, t8414, t8417)
}
