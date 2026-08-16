//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 611/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk611(t539: f64, t933: f64, t1275: f64, t331: f64, t1269: f64, t177: f64, t504: f64, t191: f64, t1244: f64, t259: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3508 = t933 * t539;
    let t3510 = t331 * t1275;
    let t3512 = t331 * t1269;
    let t3515 = 1.0_f64 / t177 / t504;
    let t3516 = t191 * t3515;
    let t3518 = 1.0_f64 / t1244 / t259;
    (t3508, t3510, t3512, t3515, t3516, t3518)
}
