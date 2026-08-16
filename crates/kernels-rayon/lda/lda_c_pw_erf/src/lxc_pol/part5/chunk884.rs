//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 884/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk884(t1023: f64, t1054: f64, t400: f64, t1012: f64, t2946: f64, t3111: f64, t174: f64, t3149: f64, t998: f64, t155: f64, t3127: f64, t3131: f64) -> (f64, f64, f64, f64) {
    let t8382 = 21.053604230838733_f64 * t400 * t1054 * t1023;
    let t8386 = 623.3672123775311_f64 * t400 * t2946 * t1012 * t3111;
    let t8389 = 0.07123333333333333_f64 * t174 * t998 * t3149;
    let t8393 = 36.84545214203136_f64 * t174 * t155 * t3127 * t3131;
    (t8382, t8386, t8389, t8393)
}
