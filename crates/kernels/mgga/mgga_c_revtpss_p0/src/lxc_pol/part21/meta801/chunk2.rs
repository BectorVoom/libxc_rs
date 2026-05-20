//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2909/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2909<F: Float>(t52035: F, t52037: F, t41308: F, t41330: F, t41332: F, t41334: F, t41336: F, t41365: F, t41367: F, t52039: F, t52041: F, t52045: F, t52047: F, t52049: F, t52051: F, t52054: F, t52057: F, t52060: F, t52063: F, t52112: F) -> F {
    let t52783 = F::cast_from(0.47488888888888888888e-1_f64) * t52035;
    let t52784 = F::cast_from(0.15829629629629629629e-1_f64) * t52037;
    let t52803 = t52783 - t52784 - F::cast_from(0.71233333333333333332e-1_f64) * t52039 - F::cast_from(0.35616666666666666666e-1_f64) * t52041 - F::cast_from(0.71233333333333333331e-1_f64) * t52045 + F::cast_from(0.23744444444444444444e-1_f64) * t52047 + F::cast_from(0.11872222222222222222e-1_f64) * t52049 + F::cast_from(0.19787037037037037036e-1_f64) * t52051 - F::cast_from(0.35616666666666666666e-1_f64) * t52054 - F::cast_from(0.35616666666666666666e-1_f64) * t52057 - F::cast_from(0.5936111111111111111e-1_f64) * t52060 - F::new(0.32055e0) * t52063 - F::cast_from(0.35616666666666666666e-1_f64) * t41365 + F::cast_from(0.11872222222222222222e-1_f64) * t41367 + F::cast_from(0.35616666666666666666e-1_f64) * t41308 - F::cast_from(0.23744444444444444444e-1_f64) * t41330 - F::cast_from(0.15829629629629629629e-1_f64) * t41332 + F::cast_from(0.5936111111111111111e-2_f64) * t41334 + F::cast_from(0.65956790123456790122e-2_f64) * t41336 - F::new(0.32055e0) * t52112;
    t52803
}
