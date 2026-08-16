//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2909/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2909(t52035: f64, t52037: f64, t41308: f64, t41330: f64, t41332: f64, t41334: f64, t41336: f64, t41365: f64, t41367: f64, t52039: f64, t52041: f64, t52045: f64, t52047: f64, t52049: f64, t52051: f64, t52054: f64, t52057: f64, t52060: f64, t52063: f64, t52112: f64) -> f64 {
    let t52783 = 0.47488888888888888888e-1_f64 * t52035;
    let t52784 = 0.15829629629629629629e-1_f64 * t52037;
    let t52803 = t52783 - t52784 - 0.71233333333333333332e-1_f64 * t52039 - 0.35616666666666666666e-1_f64 * t52041 - 0.71233333333333333331e-1_f64 * t52045 + 0.23744444444444444444e-1_f64 * t52047 + 0.11872222222222222222e-1_f64 * t52049 + 0.19787037037037037036e-1_f64 * t52051 - 0.35616666666666666666e-1_f64 * t52054 - 0.35616666666666666666e-1_f64 * t52057 - 0.5936111111111111111e-1_f64 * t52060 - 0.32055e0_f64 * t52063 - 0.35616666666666666666e-1_f64 * t41365 + 0.11872222222222222222e-1_f64 * t41367 + 0.35616666666666666666e-1_f64 * t41308 - 0.23744444444444444444e-1_f64 * t41330 - 0.15829629629629629629e-1_f64 * t41332 + 0.5936111111111111111e-2_f64 * t41334 + 0.65956790123456790122e-2_f64 * t41336 - 0.32055e0_f64 * t52112;
    t52803
}
