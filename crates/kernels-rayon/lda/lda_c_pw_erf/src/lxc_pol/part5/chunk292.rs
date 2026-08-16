//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 292/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk292(t973: f64, t991: f64, t31: f64, t4: f64, t474: f64, t155: f64, t318: f64, t174: f64, t335: f64, t379: f64, t378: f64, t80: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t992 = t973 * t991;
    let t996 = t4 * t474 * t31;
    let t997 = 0.0014764770444444443_f64 * t996;
    let t998 = t155 * t318;
    let t1000 = t174 * t998 * t335;
    let t1001 = 0.035616666666666665_f64 * t1000;
    let t1005 = t155 * t379;
    let t1009 = t378 * t80;
    (t992, t997, t998, t1001, t1005, t1009)
}
