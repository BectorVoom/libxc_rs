//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1080/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1080(t76119: f64, t76122: f64, t76127: f64, t76130: f64, t76132: f64, t71804: f64, t76103: f64, t76108: f64, t78486: f64, t78487: f64, t78488: f64, t78491: f64, t78493: f64, t78495: f64, t78497: f64, t78498: f64, t78499: f64) -> f64 {
    let t78500 = 0.17961362552795712846e0_f64 * t76119;
    let t78501 = 0.44903406381989282115e-1_f64 * t76122;
    let t78502 = 0.30487649791575028312e-3_f64 * t76127;
    let t78503 = 0.72042316457491791901e-3_f64 * t76130;
    let t78504 = 0.85129199786595678799e-5_f64 * t76132;
    let t78505 = -t78486 + t78487 - t71804 - t78488 - 0.58171619854173713846e-5_f64 * t76103 - 0.21814357445315142692e-4_f64 * t76108 - t78491 + t78493 - t78495 - t78497 + t78498 + t78499 + t78500 + t78501 - t78502 - t78503 + t78504;
    t78505
}
