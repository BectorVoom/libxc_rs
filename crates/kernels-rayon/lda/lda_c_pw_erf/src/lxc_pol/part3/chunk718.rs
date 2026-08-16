//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 718/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk718(t1309: f64, t4508: f64, t4506: f64, t3433: f64, t4464: f64, t4466: f64, t4468: f64, t4470: f64, t4471: f64, t4472: f64, t4473: f64, t4474: f64, t4478: f64, t4482: f64, t4486: f64, t4493: f64, t4499: f64, t4504: f64) -> (f64, f64, f64, f64) {
    let t4509 = t4508 * t1309;
    let t4511 = 16.0_f64 / 45.0_f64 * t4506 * t4509;
    let t4512 = 8.0_f64 / 135.0_f64 * t3433;
    let t4513 = -t4464 - t4466 + t4468 + t4470 + t4471 + t4472 + t4473 + t4474 - t4478 - t4482 - t4486 + t4493 + t4499 - t4504 + t4511 - t4512;
    (t4509, t4511, t4512, t4513)
}
