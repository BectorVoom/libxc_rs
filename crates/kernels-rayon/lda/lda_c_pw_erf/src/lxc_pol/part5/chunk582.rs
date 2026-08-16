//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 582/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk582(t265: f64, t3704: f64, t219: f64, t3604: f64, t197: f64, t3476: f64, t155: f64, t573: f64) -> (f64, f64, f64, f64) {
    let t3706 = 8.0_f64 / 405.0_f64 * t265 * t3704;
    let t3714 = t219 * t3604;
    let t3722 = t197 * t3476;
    let t3762 = t155 * t573;
    (t3706, t3714, t3722, t3762)
}
