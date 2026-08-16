//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 626/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk626(t2024: f64, t8377: f64, t739: f64, t5144: f64, t5267: f64, t884: f64, t5888: f64, t7703: f64, t1356: f64, t1632: f64, t665: f64, t903: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8378 = t2024 * t8377;
    let t8379 = t739 * t8378;
    let t8384 = t2024 * t5144;
    let t8385 = t739 * t8384;
    let t8387 = t2024 * t5267;
    let t8388 = t884 * t8387;
    let t8390 = t7703 * t5888;
    let t8391 = t1356 * t8390;
    let t8393 = t665 * t1632;
    let t8394 = t903 * t8393;
    (t8378, t8379, t8384, t8385, t8387, t8388, t8390, t8391, t8393, t8394)
}
