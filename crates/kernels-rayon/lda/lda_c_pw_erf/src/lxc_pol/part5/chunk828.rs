//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 828/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk828(t1440: f64, t7600: f64, t1325: f64, t7002: f64, t806: f64, t519: f64, t575: f64, t7370: f64, t574: f64, t571: f64, t4050: f64, t7365: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7601 = t1440 * t7600;
    let t7603 = 4.0_f64 / 5.0_f64 * t1325 * t7601;
    let t7604 = t7002 * t806;
    let t7605 = t1440 * t7604;
    let t7607 = 4.0_f64 / 5.0_f64 * t519 * t7605;
    let t7608 = t575 * t7370;
    let t7609 = t574 * t7608;
    let t7611 = 4.0_f64 / 45.0_f64 * t571 * t7609;
    let t7612 = t4050 * t7365;
    (t7601, t7603, t7604, t7605, t7607, t7608, t7609, t7611, t7612)
}
