//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1244/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1244(t14566: f64, t14567: f64, t14568: f64, t14569: f64, t14570: f64, t8946: f64, t8981: f64, t8983: f64, t8991: f64, t8995: f64, t8999: f64, t117: f64, t174: f64) -> (f64, f64) {
    let t14771 = t8946 / 6.0_f64 - 1.46904_f64 * t8981 + 0.73452_f64 * t8983 + t14566 + 5.87616_f64 * t8991 - 2.93808_f64 * t8995 + 5.87616_f64 * t8999 + t14567 - t14568 + t14569 + t14570;
    let t14777 = t117 * t174;
    (t14771, t14777)
}
