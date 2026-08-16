//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 614/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk614(t3507: f64, t3544: f64, t530: f64, t186: f64, t185: f64, t1518: f64, t495: f64, t493: f64, t543: f64, t1279: f64, t514: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3545 = t3507 + t3544;
    let t3546 = t530 * t3545;
    let t3547 = t186 * t3546;
    let t3549 = 2.0_f64 / 15.0_f64 * t185 * t3547;
    let t3550 = t1518 * t495;
    let t3551 = t493 * t3550;
    let t3552 = 8.0_f64 / 45.0_f64 * t3551;
    let t3553 = t1518 * t543;
    let t3554 = t185 * t3553;
    let t3555 = 4.0_f64 / 45.0_f64 * t3554;
    let t3556 = t514 * t1279;
    let t3557 = t185 * t3556;
    (t3545, t3546, t3547, t3549, t3550, t3551, t3552, t3553, t3554, t3555, t3556, t3557)
}
