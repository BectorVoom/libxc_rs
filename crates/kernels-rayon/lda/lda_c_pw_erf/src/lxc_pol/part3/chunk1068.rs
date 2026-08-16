//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1068/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1068(t1621: f64, t1931: f64, t4233: f64, t838: f64, t4714: f64, t611: f64, t348: f64, t494: f64, t4495: f64, t3965: f64, t4501: f64, t4494: f64) -> (f64, f64, f64, f64, f64) {
    let t12507 = t1931 * t1621;
    let t12508 = 4.0_f64 * t12507;
    let t12509 = t838 * t4233;
    let t12514 = t4714 * t611;
    let t12516 = t348 * t494;
    let t12517 = t4495 * t12516;
    let t12520 = 16.0_f64 / 9.0_f64 * t3965 * t4501 * t12517;
    let t12523 = 32.0_f64 / 15.0_f64 * t3965 * t4494 * t12517;
    (t12508, t12509, t12514, t12520, t12523)
}
