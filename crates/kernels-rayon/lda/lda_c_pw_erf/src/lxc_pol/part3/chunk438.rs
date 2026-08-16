//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 438/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk438(t119: f64, t155: f64, t411: f64, t1657: f64, t416: f64, t925: f64) -> (f64, f64, f64, f64, f64) {
    let t1659 = t119 * t155 * t411;
    let t1660 = t1657 * t1659;
    let t1661 = 0.9743416666666667_f64 * t1660;
    let t1663 = 0.6495611111111111_f64 * t416 * t925;
    let t1664 = t411 * t411;
    (t1659, t1660, t1661, t1663, t1664)
}
