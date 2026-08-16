//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 575/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk575(t506: f64, t925: f64, t1458: f64, t56: f64, t1124: f64, t174: f64, t177: f64, t1518: f64, t495: f64, t493: f64, t543: f64, t185: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3530 = t925 * t506;
    let t3536 = t56 * t1458;
    let t3540 = t1124 * t56;
    let t3542 = t174 * t3540 * t177;
    let t3543 = 0.11197407407407407_f64 * t3542;
    let t3550 = t1518 * t495;
    let t3551 = t493 * t3550;
    let t3553 = t1518 * t543;
    let t3554 = t185 * t3553;
    (t3530, t3536, t3540, t3542, t3543, t3550, t3551, t3553, t3554)
}
