//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 314/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk314(t1055: f64, t400: f64, t1036: f64, t1038: f64, t1041: f64, t1043: f64, t1045: f64, t1047: f64, t1049: f64, t1053: f64, t916: f64, t962: f64) -> (f64, f64) {
    let t1056 = t400 * t1055;
    let t1057 = 17.315755899375862_f64 * t1056;
    let t1058 = t962 + t1036 + t1038 + t1041 - t1043 - t1045 + t1047 + t1049 - t916 - t1053 - t1057;
    (t1057, t1058)
}
