//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1094/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1094(t3794: f64, t5292: f64, t1325: f64, t2098: f64, t494: f64, t5289: f64, t542: f64, t5040: f64, t518: f64, t2002: f64, t5044: f64, t1987: f64, t3745: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12788 = 16.0_f64 / 5.0_f64 * t3794 * t5292;
    let t12793 = 16.0_f64 / 5.0_f64 * t1325 * t5289 * t2098 * t494 * t542;
    let t12794 = t5040 * t518;
    let t12796 = 8.0_f64 / 15.0_f64 * t12794 * t2002;
    let t12797 = t5044 * t518;
    let t12799 = 16.0_f64 / 15.0_f64 * t12797 * t2002;
    let t12801 = 16.0_f64 / 15.0_f64 * t3745 * t1987;
    (t12788, t12793, t12794, t12796, t12797, t12799, t12801)
}
