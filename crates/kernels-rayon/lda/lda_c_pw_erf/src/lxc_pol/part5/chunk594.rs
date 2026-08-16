//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 594/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk594(t646: f64, t695: f64, t1198: f64, t1426: f64, t458: f64, t108: f64, t492: f64, t267: f64) -> (f64, f64, f64, f64, f64) {
    let t3959 = 0.06649088888888889_f64 * t695 * t646;
    let t3960 = t1198 * t646;
    let t3963 = 0.09973633333333333_f64 * t458 * t1426;
    let t3964 = t492 * t108;
    let t3965 = t3964 * t267;
    (t3959, t3960, t3963, t3964, t3965)
}
