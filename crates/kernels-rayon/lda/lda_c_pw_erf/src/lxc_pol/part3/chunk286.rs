//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 286/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk286(t11: f64, t19: f64, t919: f64, t328: f64, t922: f64, t21: f64, t635: f64) -> (f64, f64, f64, f64) {
    let t927 = 1.0_f64/f64::sqrt(t11);
    let t928 = t927 * t19;
    let t929 = t928 * t919;
    let t931 = t328 * t922;
    let t933 = t21 * t635;
    (t928, t929, t931, t933)
}
