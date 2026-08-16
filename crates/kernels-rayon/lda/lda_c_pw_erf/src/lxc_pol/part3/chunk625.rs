//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 625/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk625(t213: f64, t3667: f64, t1403: f64, t593: f64, t186: f64, t211: f64, t528: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3668 = t213 * t3667;
    let t3669 = t1403 * t593;
    let t3670 = t3668 * t3669;
    let t3671 = t186 * t3670;
    let t3673 = 4.0_f64 / 5.0_f64 * t211 * t3671;
    let t3674 = t528 * t528;
    let t3675 = 1.0_f64 / t3674;
    (t3669, t3670, t3671, t3673, t3674, t3675)
}
