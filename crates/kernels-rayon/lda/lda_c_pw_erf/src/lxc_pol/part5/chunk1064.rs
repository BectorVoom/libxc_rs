//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1064/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1064(t8189: f64, t2329: f64, t348: f64, t462: f64, t39: f64, t8327: f64) -> (f64, f64, f64, f64) {
    let t19987 = 0.5848223397455204_f64 * t8189;
    let t19994 = t2329 * t348;
    let t19997 = t462 * t2329;
    let t20007 = 12.0_f64 * t39 + 24.0_f64 * t8327;
    (t19987, t19994, t19997, t20007)
}
