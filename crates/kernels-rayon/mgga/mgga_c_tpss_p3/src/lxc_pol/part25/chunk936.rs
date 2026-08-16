//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 936/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk936(t11004: f64, t10982: f64, t3800: f64, t673: f64, t3797: f64, t11002: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11005 = 4.0_f64 / 9.0_f64 * t11004;
    let t11006 = 2.0_f64 / 9.0_f64 * t10982;
    let t11049 = t673 * t3800;
    let t11050 = 0.21908444444444444444e0_f64 * t11049;
    let t11051 = t673 * t3797;
    let t11071 = 0.39862222222222222222e0_f64 * t11004;
    let t11109 = 0.41203703703703703704e-2_f64 * t11002;
    let t11110 = 0.12361111111111111111e-1_f64 * t11004;
    let t11111 = 0.61805555555555555556e-2_f64 * t10982;
    let t11134 = 0.23744444444444444444e-1_f64 * t11004;
    let t11135 = 0.11872222222222222222e-1_f64 * t10982;
    let t11169 = 0.20128333333333333334e0_f64 * t10982;
    (t11005, t11006, t11049, t11050, t11051, t11071, t11109, t11110, t11111, t11134, t11135, t11169)
}
