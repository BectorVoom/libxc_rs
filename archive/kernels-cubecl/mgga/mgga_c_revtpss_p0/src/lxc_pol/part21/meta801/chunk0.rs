//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2907/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2907<F: Float>(t52126: F, t41308: F, t41330: F, t41332: F, t41334: F, t41336: F, t41365: F, t41367: F, t41441: F, t52112: F, t52128: F, t52130: F) -> F {
    let t52751 = F::cast_from(0.27385555555555555556e0_f64) * t52126;
    let t52756 = -F::cast_from(0.59793333333333333333e0_f64) * t41365 + F::cast_from(0.19931111111111111112e0_f64) * t41367 + F::cast_from(0.59793333333333333333e0_f64) * t41308 - F::cast_from(0.39862222222222222224e0_f64) * t41330 - F::cast_from(0.26574814814814814816e0_f64) * t41332 + F::cast_from(0.99655555555555555557e-1_f64) * t41334 + F::cast_from(0.11072839506172839506e0_f64) * t41336 - t52751 + F::cast_from(0.24342716049382716049e0_f64) * t52128 + F::cast_from(0.1898925e1_f64) * t52130 - F::cast_from(0.53814e1_f64) * t52112 + F::cast_from(0.73028148148148148149e0_f64) * t41441;
    t52756
}
