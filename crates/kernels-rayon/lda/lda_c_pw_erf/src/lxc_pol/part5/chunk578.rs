//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 578/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk578(t560: f64, t925: f64, t1484: f64, t56: f64, t174: f64, t205: f64, t3540: f64, t1518: f64, t550: f64, t548: f64, t594: f64, t211: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3627 = t925 * t560;
    let t3633 = t56 * t1484;
    let t3638 = t174 * t3540 * t205;
    let t3639 = 0.11197407407407407_f64 * t3638;
    let t3660 = t1518 * t550;
    let t3661 = t548 * t3660;
    let t3663 = t1518 * t594;
    let t3664 = t211 * t3663;
    (t3627, t3633, t3638, t3639, t3660, t3661, t3663, t3664)
}
