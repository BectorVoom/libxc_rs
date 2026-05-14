//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 959/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk959<F: Float>(t35707: F, t35720: F, t35724: F, t35742: F, t35744: F, t37821: F, t37822: F, t37825: F, t43433: F, t43440: F, t47030: F, t47032: F, t47037: F, t47042: F, t47048: F, t47054: F, t6304: F, t708: F) -> (F,) {
    let t48864 = 0.60975299583150056624e-3 * t35707 + t37821 + t37822 - 0.86737941314158990616e-4 * t35720 - 0.86737941314158990616e-4 * t35724 - t37825 + 0.30487649791575028312e-3 * t35742 + 0.30487649791575028312e-3 * t35744 + t43433 - 0.1064114997332445985e-4 * t47030 - 0.19957069503106347607e-1 * t6304 * t708 + 0.49658699875514145967e-4 * t47032 - t43440 - 0.2553875993597870364e-3 * t47037 - 0.638468998399467591e-4 * t47042 + 0.638468998399467591e-4 * t47048 - 0.10215503974391481456e-3 * t47054;
    (t48864,)
}
