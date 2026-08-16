//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1100/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1100(t35707: f64, t35720: f64, t35724: f64, t35742: f64, t35744: f64, t37821: f64, t37822: f64, t37825: f64, t43433: f64, t43440: f64, t47030: f64, t47032: f64, t47037: f64, t47042: f64, t47048: f64, t47054: f64, t6304: f64, t708: f64) -> f64 {
    let t48864 = 0.60975299583150056624e-3_f64 * t35707 + t37821 + t37822 - 0.86737941314158990616e-4_f64 * t35720 - 0.86737941314158990616e-4_f64 * t35724 - t37825 + 0.30487649791575028312e-3_f64 * t35742 + 0.30487649791575028312e-3_f64 * t35744 + t43433 - 0.1064114997332445985e-4_f64 * t47030 - 0.19957069503106347607e-1_f64 * t6304 * t708 + 0.49658699875514145967e-4_f64 * t47032 - t43440 - 0.2553875993597870364e-3_f64 * t47037 - 0.638468998399467591e-4_f64 * t47042 + 0.638468998399467591e-4_f64 * t47048 - 0.10215503974391481456e-3_f64 * t47054;
    t48864
}
