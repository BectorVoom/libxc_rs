//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 817/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk817(t571: f64, t7489: f64, t2146: f64, t2562: f64, t6212: f64, t6216: f64, t6221: f64, t6223: f64, t7460: f64, t7462: f64, t7464: f64, t7468: f64, t7472: f64, t7473: f64, t7477: f64, t7481: f64, t7483: f64, t7487: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7491 = 8.0_f64 / 9.0_f64 * t571 * t7489;
    let t7493 = 8.0_f64 / 15.0_f64 * t2146 * t2562;
    let t7494 = 16.0_f64 / 15.0_f64 * t6212;
    let t7495 = 8.0_f64 / 15.0_f64 * t6216;
    let t7496 = 8.0_f64 / 15.0_f64 * t6221;
    let t7497 = 8.0_f64 / 15.0_f64 * t6223;
    let t7498 = -t7460 - t7462 - t7464 + t7468 + t7472 - t7473 + t7477 + t7481 - t7483 - t7487 - t7491 - t7493 + t7494 + t7495 + t7496 - t7497;
    (t7491, t7493, t7494, t7495, t7496, t7497, t7498)
}
