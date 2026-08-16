//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 627/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk627(t1635: f64, t665: f64, t1364: f64, t2024: f64, t5898: f64, t884: f64, t2060: f64, t5144: f64, t1550: f64, t5267: f64, t903: f64, t1627: f64, t645: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8396 = t665 * t1635;
    let t8397 = t1364 * t8396;
    let t8399 = t2024 * t5898;
    let t8400 = t884 * t8399;
    let t8404 = t2060 * t5144;
    let t8405 = t1550 * t8404;
    let t8407 = t2060 * t5267;
    let t8408 = t903 * t8407;
    let t8410 = t645 * t1627;
    (t8396, t8397, t8399, t8400, t8404, t8405, t8407, t8408, t8410)
}
