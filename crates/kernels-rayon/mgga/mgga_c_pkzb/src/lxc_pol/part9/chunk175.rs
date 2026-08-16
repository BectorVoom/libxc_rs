//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 175/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk175(t110: f64, t123: f64, t466: f64, t469: f64, t49: f64, t494: f64, t520: f64, t527: f64, t535: f64, t542: f64) -> f64 {
    let t545 = 0.53237641966666666666e-3_f64 * t49 * t466 * t110 + 1.0_f64 * t520 * t527 - t469 - t494 + 0.18311447306006545054e-3_f64 * t49 * t466 * t123 + 0.5848223622634646207e0_f64 * t535 * t542;
    t545
}
