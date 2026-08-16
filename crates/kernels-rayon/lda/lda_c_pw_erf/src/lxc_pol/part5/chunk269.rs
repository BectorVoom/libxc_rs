//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 269/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk269(t571: f64, t826: f64, t589: f64, t816: f64, t25: f64, t583: f64, t587: f64, t818: f64) -> (f64, f64, f64) {
    let t828 = 4.0_f64 / 45.0_f64 * t571 * t826;
    let t830 = t589 * t816;
    let t833 = -t583 - 0.035991666666666665_f64 * t818 - t587 - 0.006666666666666667_f64 * t25 * t830;
    (t828, t830, t833)
}
