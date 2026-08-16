//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 160/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk160(t103: f64, t415: f64, t325: f64, t102: f64, t120: f64, t411: f64, t118: f64, t119: f64, t155: f64, t117: f64, t4: f64) -> (f64, f64, f64, f64, f64) {
    let t416 = t415 * t103;
    let t418 = 0.48717083333333333_f64 * t416 * t325;
    let t421 = 2.923025_f64 * t102 * t120 * t411;
    let t425 = t118 * t119 * t155 * t120 / 12.0_f64;
    let t426 = t117 * t4;
    (t416, t418, t421, t425, t426)
}
