//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 907/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk907(t3619: f64, t3854: f64, t571: f64, t1318: f64, t3429: f64, t4794: f64, t4062: f64, t581: f64, t3833: f64, t3667: f64, t574: f64, t1390: f64, t1449: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9273 = t571 * t3854 * t3619;
    let t9276 = t1318 * t4794 * t3429;
    let t9278 = t4062 * t581;
    let t9280 = t571 * t9278 * t3833;
    let t9286 = t574 * t3667;
    let t9304 = t1449 * t1390;
    (t9273, t9276, t9278, t9280, t9286, t9304)
}
