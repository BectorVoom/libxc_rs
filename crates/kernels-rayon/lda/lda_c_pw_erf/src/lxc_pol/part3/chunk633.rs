//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 633/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk633(t3738: f64, t522: f64, t519: f64, t1523: f64, t518: f64) -> (f64, f64, f64) {
    let t3739 = t522 * t3738;
    let t3741 = 8.0_f64 / 15.0_f64 * t519 * t3739;
    let t3742 = t1523 * t518;
    (t3739, t3741, t3742)
}
