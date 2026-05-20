//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2927/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2927<F: Float>(t52035: F, t52037: F, t41308: F, t41330: F, t41332: F, t41334: F, t41336: F, t41365: F, t41367: F, t52039: F, t52041: F, t52045: F, t52047: F, t52049: F, t52051: F, t52054: F, t52057: F, t52060: F, t52063: F, t52112: F) -> F {
    let t53252 = F::cast_from(0.39511111111111111112e-1_f64) * t52035;
    let t53253 = F::cast_from(0.13170370370370370371e-1_f64) * t52037;
    let t53272 = t53252 - t53253 - F::cast_from(0.59266666666666666668e-1_f64) * t52039 - F::cast_from(0.29633333333333333334e-1_f64) * t52041 - F::cast_from(0.59266666666666666669e-1_f64) * t52045 + F::cast_from(0.19755555555555555556e-1_f64) * t52047 + F::cast_from(0.98777777777777777781e-2_f64) * t52049 + F::cast_from(0.16462962962962962963e-1_f64) * t52051 - F::cast_from(0.29633333333333333334e-1_f64) * t52054 - F::cast_from(0.29633333333333333334e-1_f64) * t52057 - F::cast_from(0.4938888888888888889e-1_f64) * t52060 - F::new(0.2667e0) * t52063 - F::cast_from(0.29633333333333333334e-1_f64) * t41365 + F::cast_from(0.98777777777777777781e-2_f64) * t41367 + F::cast_from(0.29633333333333333334e-1_f64) * t41308 - F::cast_from(0.19755555555555555556e-1_f64) * t41330 - F::cast_from(0.13170370370370370371e-1_f64) * t41332 + F::cast_from(0.4938888888888888889e-2_f64) * t41334 + F::cast_from(0.54876543209876543212e-2_f64) * t41336 - F::new(0.2667e0) * t52112;
    t53272
}
