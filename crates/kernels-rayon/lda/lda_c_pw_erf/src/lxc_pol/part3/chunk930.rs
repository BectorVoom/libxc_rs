//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 930/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk930(t1155: f64, t603: f64, t230: f64, t4222: f64, t331: f64, t3615: f64, t10042: f64, t3606: f64, t2061: f64, t590: f64, t1375: f64, t933: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10172 = 0.004413481481481482_f64 * t1155 * t603;
    let t10173 = t4222 * t230;
    let t10178 = t331 * t3615;
    let t10195 = 0.3732469135802469_f64 * t10042;
    let t10196 = t331 * t3606;
    let t10202 = t2061 * t590;
    let t10204 = t933 * t1375;
    (t10172, t10173, t10178, t10195, t10196, t10202, t10204)
}
