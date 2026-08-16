//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 636/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk636(t3757: f64, t571: f64, t1472: f64, t1476: f64, t155: f64, t573: f64) -> (f64, f64, f64, f64) {
    let t3759 = 4.0_f64 / 5.0_f64 * t571 * t3757;
    let t3760 = t1472 * t1476;
    let t3761 = 16.0_f64 / 45.0_f64 * t3760;
    let t3762 = t155 * t573;
    (t3759, t3760, t3761, t3762)
}
