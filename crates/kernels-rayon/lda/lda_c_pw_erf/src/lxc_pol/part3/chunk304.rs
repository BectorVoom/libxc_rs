//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 304/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk304(t1012: f64, t386: f64, t920: f64, t923: f64, t925: f64, t929: f64, t931: f64, t933: f64) -> (f64, f64) {
    let t1013 = t1012 * t386;
    let t1022 = -0.5753888888888888_f64 * t920 + 1.1507777777777777_f64 * t923 + 0.4025666666666667_f64 * t925 + 0.0366775_f64 * t929 + 0.073355_f64 * t931 + 0.137975_f64 * t933;
    (t1013, t1022)
}
