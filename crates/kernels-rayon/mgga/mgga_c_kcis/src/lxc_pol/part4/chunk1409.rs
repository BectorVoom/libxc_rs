//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1409/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1409(t17484: f64, t17488: f64, t17491: f64, t17494: f64, t17497: f64, t17499: f64, t17502: f64, t17506: f64, t17510: f64, t17512: f64, t17515: f64, t17518: f64, t17521: f64, t17693: f64, t17695: f64, t17698: f64, t17700: f64, t17704: f64) -> f64 {
    let t18350 = 0.101171875e-1_f64 * t17484 - 0.9375e-1_f64 * t17488 + 0.125e0_f64 * t17491 - 0.16666666666666666667e0_f64 * t17494 + 0.25e0_f64 * t17497 + 0.14388888888888888889e0_f64 * t17499 - 0.53958333333333333333e-1_f64 * t17502 - 0.14388888888888888889e0_f64 * t17506 - 0.5625e0_f64 * t17510 - 0.13489583333333333333e-1_f64 * t17512 + 0.20234375e-1_f64 * t17515 - 0.20234375e-1_f64 * t17518 + 0.55555555555555555557e-1_f64 * t17521 + 0.9375e-1_f64 * t17693 + 0.101171875e-1_f64 * t17695 + 0.41666666666666666666e-1_f64 * t17698 - 0.25e0_f64 * t17700 + 0.101171875e-1_f64 * t17704;
    t18350
}
