//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 220/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk220(t559: f64, t589: f64, t25: f64, t561: f64, t583: f64, t587: f64) -> (f64, f64) {
    let t590 = t589 * t559;
    let t593 = -t583 - 0.035991666666666665_f64 * t561 - t587 - 0.006666666666666667_f64 * t25 * t590;
    (t590, t593)
}
