//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1159/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1159(t16762: f64, t16765: f64, t16768: f64, t16819: f64, t16829: f64, t16874: f64, t12197: f64, t12310: f64, t21249: f64, t21251: f64, t21255: f64, t21257: f64, t21261: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21262 = 16.0_f64 / 45.0_f64 * t16762;
    let t21263 = 16.0_f64 / 15.0_f64 * t16765;
    let t21264 = 16.0_f64 / 9.0_f64 * t16768;
    let t21265 = 32.0_f64 / 45.0_f64 * t16819;
    let t21266 = 32.0_f64 / 45.0_f64 * t16829;
    let t21267 = 32.0_f64 / 45.0_f64 * t16874;
    let t21268 = -t21249 + t21251 - t21255 - t21257 - t21261 + t21262 + t21263 - t21264 + t12197 + t12310 + t21265 - t21266 + t21267;
    (t21262, t21263, t21264, t21265, t21266, t21267, t21268)
}
