//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 442/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk442(t1659: f64, t1691: f64, t432: f64, t925: f64, t435: f64, t95: f64) -> (f64, f64, f64) {
    let t1692 = t1691 * t1659;
    let t1695 = 0.3264533333333333_f64 * t432 * t925;
    let t1697 = 1.0_f64 / t435 / t95;
    (t1692, t1695, t1697)
}
