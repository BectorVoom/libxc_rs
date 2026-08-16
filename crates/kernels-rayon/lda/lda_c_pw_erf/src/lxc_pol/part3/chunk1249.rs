//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1249/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1249(t14584: f64, t426: f64, t325: f64, t431: f64, t5565: f64, t1686: f64, t1856: f64, t933: f64, t14587: f64, t127: f64, t14632: f64, t14684: f64, t14686: f64, t14689: f64, t14692: f64, t14695: f64, t14719: f64, t436: f64) -> f64 {
    let t14843 = t426 * t14584;
    let t14844 = 2.0_f64 / 3.0_f64 * t14843;
    let t14846 = t431 * t5565 * t325;
    let t14849 = t1686 * t1856 * t933;
    let t14850 = 1.46904_f64 * t14849;
    let t14851 = t426 * t14587;
    let t14853 = -t14684 - t14686 + t14689 + t14692 - 1.46904_f64 * t127 * t436 * t14632 - t14695 - t14844 + 2.20356_f64 * t14846 - t14850 + t14851 / 2.0_f64 + t14719;
    t14853
}
