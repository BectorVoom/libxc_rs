//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1382/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1382(t187: f64, t95514: f64, t95517: f64, t95520: f64, t96542: f64, t96545: f64, t96668: f64, t96689: f64, t96708: f64, t97499: f64, t97500: f64, t97501: f64, t97503: f64, t97505: f64, t97507: f64, t97510: f64, t97511: f64, t97513: f64, t97517: f64, t97521: f64, t97526: f64, t97528: f64, t97529: f64) -> f64 {
    let t97533 = t95514 + t95517 + t95520 + t96542 + t96545 + t187 * (t96668 + t96689 + t96708 + t97529) - t97499 - t97500 - t97501 - t97503 + t97505 + t97507 + t97510 - t97511 - t97513 + t97517 + t97521 - t97526 + t97528;
    t97533
}
