//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 527/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk527(t143: f64, t2610: f64, t1611: f64, t1623: f64, t1927: f64, t2387: f64, t2391: f64, t2395: f64, t2399: f64, t2404: f64, t2409: f64, t2427: f64, t2445: f64) -> (f64, f64) {
    let t2647 = t143 * t2610;
    let t2657 = t1611 + t2387 - t2391 + t2395 - t2399 + t1623 + 0.21642082724729686_f64 * t1927 + t2404 + t2409 + t2427 + t2445;
    (t2647, t2657)
}
