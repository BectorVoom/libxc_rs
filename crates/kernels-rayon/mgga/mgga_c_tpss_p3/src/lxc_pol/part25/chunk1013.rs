//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1013/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1013(t57: f64, t13335: f64, t14016: f64, t14021: f64, t3431: f64, t3602: f64, t581: f64, t745: f64, t14015: f64, zeta_threshold: f64) -> f64 {
    let t155 = t57 <= zeta_threshold;
    let t14027 = piecewise3(t155, 0.0_f64, -8.0_f64 / 27.0_f64 * t14016 * t581 - 4.0_f64 / 9.0_f64 * t3602 * t3431 - 2.0_f64 / 9.0_f64 * t14021 * t581 - 2.0_f64 / 3.0_f64 * t745 * t13335);
    let t14029 = t14015 / 2.0_f64 + t14027 / 2.0_f64;
    t14029
}
