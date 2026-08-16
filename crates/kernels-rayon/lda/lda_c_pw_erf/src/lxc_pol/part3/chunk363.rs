//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 363/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk363(t1308: f64, t1309: f64, t571: f64, t522: f64, t529: f64) -> (f64, f64, f64) {
    let t1310 = t1308 * t1309;
    let t1312 = 8.0_f64 / 45.0_f64 * t571 * t1310;
    let t1313 = t522 * t529;
    (t1310, t1312, t1313)
}
