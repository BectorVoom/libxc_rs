//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1320/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1320(t5833: f64, t668: f64, t14307: f64, t14311: f64, t14314: f64, t14317: f64, t14319: f64, t14321: f64, t14323: f64, t14327: f64, t14329: f64, t14331: f64, t14333: f64, t14338: f64) -> f64 {
    let t15204 = t5833 * t668;
    let t15206 = -2.0_f64 / 15.0_f64 * t15204 + t14307 - t14311 + t14314 - t14317 + t14319 - t14321 - t14323 - t14327 - t14329 - t14331 + t14333 - t14338;
    t15206
}
