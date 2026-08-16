//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 977/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk977(t14392: f64, t1131: f64, t485: f64, t5470: f64, t1191: f64, t780: f64, t1138: f64, t1597: f64, t5932: f64, t1904: f64, t717: f64, t2916: f64, t5466: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14393 = 0.01975389032890948_f64 * t14392;
    let t14395 = t5470 * t1131 * t485;
    let t14397 = t1191 * t780;
    let t14399 = t14397 * t1138 * t1597;
    let t14401 = t5932 * t485;
    let t14403 = t717 * t1904;
    let t14405 = t14403 * t1138 * t1597;
    let t14406 = 0.0014862827083471494_f64 * t14405;
    let t14408 = t5466 * t2916 * t1597;
    (t14393, t14395, t14397, t14399, t14401, t14403, t14406, t14408)
}
