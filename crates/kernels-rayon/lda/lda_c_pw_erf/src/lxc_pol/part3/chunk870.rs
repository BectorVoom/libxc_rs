//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 870/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk870(t1746: f64, t4307: f64, t902: f64, t906: f64, t13: f64, t8185: f64, t3128: f64, t907: f64, t3153: f64, t357: f64, t40: f64, t174: f64, t2749: f64, t936: f64) -> (f64, f64, f64, f64, f64) {
    let t8405 = t4307 * t1746;
    let t8407 = t902 * t902;
    let t8410 = t906 * t906;
    let t8414 = 24954.97798673547_f64 * t13 / t8407 * t8185 / t8410;
    let t8417 = 578.9456755974397_f64 * t3128 * t8185 * t907;
    let t8419 = t40 * t357 * t3153;
    let t8423 = 0.14246666666666666_f64 * t174 * t2749 * t936;
    (t8405, t8414, t8417, t8419, t8423)
}
