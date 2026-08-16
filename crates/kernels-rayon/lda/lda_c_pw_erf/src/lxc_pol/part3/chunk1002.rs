//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1002/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1002(t1973: f64, t925: f64, t325: f64, t4625: f64, t4611: f64, t4606: f64, t4621: f64, t11: f64, t11687: f64, t1243: f64, t11691: f64, t1953: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11709 = t925 * t1973;
    let t11711 = t325 * t4625;
    let t11713 = t325 * t4611;
    let t11715 = t4606 * t4621;
    let t11718 = t11 * t1243 * t11687;
    let t11721 = t1953 * t1243 * t11691;
    (t11709, t11711, t11713, t11715, t11718, t11721)
}
