//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 556/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk556(t13935: f64, t13938: f64, t13943: f64, t13903: f64, t13906: f64, t13929: f64, t13932: f64, t13941: f64, t14476: f64, t14477: f64, t14478: f64, t14481: f64, t14482: f64, t14483: f64, t14484: f64, t14485: f64, t14486: f64, t14487: f64) -> (f64, f64, f64) {
    let t14490 = 0.48384206071776340879e-3_f64 * t13935;
    let t14491 = 0.14464861606874801909e-3_f64 * t13938;
    let t14493 = 0.12857654761666490586e-3_f64 * t13943;
    let t14494 = t14476 - t14477 - t14478 - 0.68186654135613354322e-2_f64 * t13903 + 0.13637330827122670864e-1_f64 * t13906 + t14481 + t14482 - t14483 - t14484 + t14485 - t14486 - t14487 - 0.45360193192290319574e-3_f64 * t13929 + 0.63504270469206447404e-3_f64 * t13932 + t14490 + t14491 - 0.19286482142499735878e-3_f64 * t13941 - t14493;
    (t14490, t14493, t14494)
}
