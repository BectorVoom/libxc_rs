//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 868/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk868(t1022: f64, t2986: f64, t1012: f64, t2983: f64, t400: f64, t1: f64, t2979: f64, t397: f64, t1023: f64, t1054: f64, t2946: f64, t3111: f64) -> (f64, f64, f64, f64, f64) {
    let t8370 = t2986 * t1022;
    let t8373 = 6152.338212604677_f64 * t400 * t2983 * t1012 * t8370;
    let t8375 = t2979 * t1 * t397;
    let t8382 = 21.053604230838733_f64 * t400 * t1054 * t1023;
    let t8386 = 623.3672123775311_f64 * t400 * t2946 * t1012 * t3111;
    (t8370, t8373, t8375, t8382, t8386)
}
