//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 760/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk760(t4938: f64, t519: f64, t1401: f64, t1403: f64, t811: f64, t1466: f64, t1318: f64, t2182: f64, t3787: f64, t1325: f64, t1341: f64, t2171: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4940 = 4.0_f64 / 5.0_f64 * t519 * t4938;
    let t4942 = t1401 * t811 * t1403;
    let t4943 = t1466 * t4942;
    let t4945 = 8.0_f64 / 15.0_f64 * t1318 * t4943;
    let t4946 = t3787 * t2182;
    let t4948 = 16.0_f64 / 45.0_f64 * t1325 * t4946;
    let t4950 = 8.0_f64 / 45.0_f64 * t2171 * t1341;
    (t4940, t4942, t4943, t4945, t4946, t4948, t4950)
}
