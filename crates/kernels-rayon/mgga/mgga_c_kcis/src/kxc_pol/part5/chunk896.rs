//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 896/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk896(t7311: f64, t7315: f64, t7319: f64, t7323: f64, t7330: f64, t7333: f64, t7336: f64, t7339: f64, t7383: f64, t7387: f64, t7390: f64, t7394: f64) -> f64 {
    let t7565 = -0.20833333333333333333e-1_f64 * t7311 + 0.625e-1_f64 * t7315 - 0.20234375e-1_f64 * t7319 - 0.101171875e-1_f64 * t7323 - 0.34173611111111111111e0_f64 * t7330 + 0.14388888888888888889e0_f64 * t7333 + 0.5e0_f64 * t7336 - 0.125e0_f64 * t7339 + 0.9375e-1_f64 * t7383 + 0.91666666666666666667e0_f64 * t7387 - 0.33333333333333333334e0_f64 * t7390 + 0.1875e0_f64 * t7394;
    t7565
}
