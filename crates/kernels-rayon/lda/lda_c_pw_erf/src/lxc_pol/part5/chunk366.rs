//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 366/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk366(t1529: f64, t211: f64, t1125: f64, t153: f64, t274: f64, t474: f64, t678: f64, t450: f64, t454: f64, t142: f64, t131: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1531 = 4.0_f64 / 135.0_f64 * t211 * t1529;
    let t1540 = 1.328721022894618_f64 * t153 * t1125 * t274;
    let t1542 = t153 * t474 * t678;
    let t1549 = t454 * t450;
    let t1550 = t1549 * t142;
    let t1552 = t131 * t131;
    let t1553 = 1.0_f64 / t1552;
    (t1531, t1540, t1542, t1549, t1550, t1552, t1553)
}
