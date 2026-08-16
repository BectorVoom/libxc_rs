//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 600/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk600(t1326: f64, t3412: f64, t519: f64, t1283: f64, t518: f64) -> (f64, f64, f64) {
    let t3413 = t1326 * t3412;
    let t3415 = 8.0_f64 / 15.0_f64 * t519 * t3413;
    let t3416 = t1283 * t518;
    (t3413, t3415, t3416)
}
