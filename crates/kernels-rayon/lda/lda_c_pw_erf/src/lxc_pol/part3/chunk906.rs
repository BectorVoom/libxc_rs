//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 906/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk906(t1498: f64, t1529: f64, t1612: f64, t1621: f64, t4222: f64, t611: f64, t1318: f64, t3424: f64, t3854: f64, t3824: f64, t3863: f64, t571: f64) -> (f64, f64, f64, f64, f64) {
    let t9251 = t1498 * t1529;
    let t9253 = t1612 * t1621;
    let t9259 = t4222 * t611;
    let t9267 = t1318 * t3854 * t3424;
    let t9270 = t571 * t3863 * t3824;
    (t9251, t9253, t9259, t9267, t9270)
}
