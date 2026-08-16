//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 382/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk382(t103: f64, t1652: f64, t933: f64, t1: f64, t120: f64, t415: f64, t119: f64, t155: f64, t411: f64, t416: f64, t925: f64, t118: f64, t473: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1653 = t1652 * t103;
    let t1655 = 0.3247805555555556_f64 * t1653 * t933;
    let t1657 = t415 * t120 * t1;
    let t1659 = t119 * t155 * t411;
    let t1660 = t1657 * t1659;
    let t1663 = 0.6495611111111111_f64 * t416 * t925;
    let t1674 = t118 * t119 * t473 * t120 / 9.0_f64;
    (t1653, t1655, t1657, t1659, t1660, t1663, t1674)
}
