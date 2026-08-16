//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 980/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk980(t13541: f64, t630: f64, t13154: f64, t13157: f64, t13159: f64, t13483: f64, t13486: f64, t13489: f64, t13492: f64, t13495: f64, t69: f64, t7587: f64, t7588: f64) -> f64 {
    let t13542 = t630 * t13541;
    let t13545 = -t7587 - 11.0_f64 / 9.0_f64 * t7588 - 22.0_f64 / 9.0_f64 * t13154 - t13157 + t13159 - 2.0_f64 / 3.0_f64 * t13483 - 3.0_f64 / 4.0_f64 * t69 * t13486 + t69 * t13489 / 2.0_f64 + t13492 / 3.0_f64 + t69 * t13495 / 4.0_f64 - t69 * t13542 / 8.0_f64;
    t13545
}
