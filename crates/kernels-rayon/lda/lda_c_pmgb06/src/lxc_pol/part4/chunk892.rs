//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 892/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk892(t1423: f64, t2501: f64, t1447: f64, t2497: f64, t4837: f64, t4845: f64, t5045: f64, t5047: f64, t4786: f64, t4788: f64, t4792: f64, t4794: f64, t4807: f64, t4809: f64, t4812: f64, t4814: f64, t4950: f64, t4970: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6423 = t1423 * t2501;
    let t6424 = 4.0_f64 / 135.0_f64 * t6423;
    let t6425 = t1447 * t2497;
    let t6426 = 4.0_f64 / 135.0_f64 * t6425;
    let t6427 = 2.0_f64 / 135.0_f64 * t4837;
    let t6428 = 2.0_f64 / 135.0_f64 * t4845;
    let t6429 = 2.0_f64 / 135.0_f64 * t5045;
    let t6430 = 2.0_f64 / 135.0_f64 * t5047;
    let t6431 = -t6424 - t6426 + t4786 + t4788 + t4792 + t4794 + t4807 + t4809 + t4812 + t4814 + t6427 + t6428 - t4950 - t4970 - t6429 - t6430;
    (t6424, t6426, t6427, t6428, t6429, t6430, t6431)
}
