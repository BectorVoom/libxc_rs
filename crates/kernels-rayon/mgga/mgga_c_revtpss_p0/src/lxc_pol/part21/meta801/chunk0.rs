//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2907/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2907(t52126: f64, t41308: f64, t41330: f64, t41332: f64, t41334: f64, t41336: f64, t41365: f64, t41367: f64, t41441: f64, t52112: f64, t52128: f64, t52130: f64) -> f64 {
    let t52751 = 0.27385555555555555556e0_f64 * t52126;
    let t52756 = -0.59793333333333333333e0_f64 * t41365 + 0.19931111111111111112e0_f64 * t41367 + 0.59793333333333333333e0_f64 * t41308 - 0.39862222222222222224e0_f64 * t41330 - 0.26574814814814814816e0_f64 * t41332 + 0.99655555555555555557e-1_f64 * t41334 + 0.11072839506172839506e0_f64 * t41336 - t52751 + 0.24342716049382716049e0_f64 * t52128 + 0.1898925e1_f64 * t52130 - 0.53814e1_f64 * t52112 + 0.73028148148148148149e0_f64 * t41441;
    t52756
}
