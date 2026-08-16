//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 379/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk379(t1381: f64, t582: f64, t186: f64, t211: f64, t495: f64, t514: f64) -> (f64, f64, f64, f64) {
    let t1382 = t582 * t1381;
    let t1383 = t186 * t1382;
    let t1385 = 2.0_f64 / 15.0_f64 * t211 * t1383;
    let t1386 = t514 * t495;
    (t1382, t1383, t1385, t1386)
}
