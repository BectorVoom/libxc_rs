//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 163/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk163(t411: f64, t436: f64, t127: f64, t418: f64, t421: f64, t425: f64, t426: f64, t428: f64, t434: f64) -> f64 {
    let t437 = t436 * t411;
    let t440 = -t418 - t421 - t425 - t426 * t428 / 2.0_f64 - t434 - 1.46904_f64 * t127 * t437;
    t440
}
