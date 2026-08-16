//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 709/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk709(t1328: f64, t4483: f64, t3965: f64, t108: f64, t182: f64, t267: f64) -> (f64, f64, f64, f64) {
    let t4484 = t4483 * t1328;
    let t4486 = 16.0_f64 / 45.0_f64 * t3965 * t4484;
    let t4487 = t182 * t108;
    let t4488 = t4487 * t267;
    (t4484, t4486, t4487, t4488)
}
