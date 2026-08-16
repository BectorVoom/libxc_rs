//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 438/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk438(t2065: f64, t582: f64, t186: f64, t211: f64, t1: f64, t473: f64) -> (f64, f64, f64, f64) {
    let t2066 = t582 * t2065;
    let t2067 = t186 * t2066;
    let t2069 = 2.0_f64 / 15.0_f64 * t211 * t2067;
    let t2070 = t1 * t473;
    (t2066, t2067, t2069, t2070)
}
