//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2927/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2927(t52035: f64, t52037: f64, t41308: f64, t41330: f64, t41332: f64, t41334: f64, t41336: f64, t41365: f64, t41367: f64, t52039: f64, t52041: f64, t52045: f64, t52047: f64, t52049: f64, t52051: f64, t52054: f64, t52057: f64, t52060: f64, t52063: f64, t52112: f64) -> f64 {
    let t53252 = 0.39511111111111111112e-1_f64 * t52035;
    let t53253 = 0.13170370370370370371e-1_f64 * t52037;
    let t53272 = t53252 - t53253 - 0.59266666666666666668e-1_f64 * t52039 - 0.29633333333333333334e-1_f64 * t52041 - 0.59266666666666666669e-1_f64 * t52045 + 0.19755555555555555556e-1_f64 * t52047 + 0.98777777777777777781e-2_f64 * t52049 + 0.16462962962962962963e-1_f64 * t52051 - 0.29633333333333333334e-1_f64 * t52054 - 0.29633333333333333334e-1_f64 * t52057 - 0.4938888888888888889e-1_f64 * t52060 - 0.2667e0_f64 * t52063 - 0.29633333333333333334e-1_f64 * t41365 + 0.98777777777777777781e-2_f64 * t41367 + 0.29633333333333333334e-1_f64 * t41308 - 0.19755555555555555556e-1_f64 * t41330 - 0.13170370370370370371e-1_f64 * t41332 + 0.4938888888888888889e-2_f64 * t41334 + 0.54876543209876543212e-2_f64 * t41336 - 0.2667e0_f64 * t52112;
    t53272
}
