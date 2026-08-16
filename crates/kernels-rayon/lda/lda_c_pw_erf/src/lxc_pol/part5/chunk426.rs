//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 426/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk426(t1967: f64, t1991: f64, t519: f64, t504: f64, t806: f64, t348: f64) -> (f64, f64, f64, f64) {
    let t1992 = t1991 * t1967;
    let t1994 = 4.0_f64 / 27.0_f64 * t519 * t1992;
    let t1995 = t806 * t504;
    let t1996 = t1995 * t348;
    (t1992, t1994, t1995, t1996)
}
