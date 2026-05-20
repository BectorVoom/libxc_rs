//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1476/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1476<F: Float>(t41341: F, t41344: F, t41347: F, t41350: F, t41353: F, t41356: F, t41359: F, t41361: F, t41363: F, t41365: F, t41367: F, t41369: F) -> F {
    let t41926 = -F::cast_from(0.50735802469135802467e-1_f64) * t41341 - F::cast_from(0.17123333333333333333e-1_f64) * t41344 - F::cast_from(0.41095999999999999999e0_f64) * t41347 + F::cast_from(0.2283111111111111111e0_f64) * t41350 - F::cast_from(0.11415555555555555555e0_f64) * t41353 + F::cast_from(0.13698666666666666667e0_f64) * t41356 - F::cast_from(0.4566222222222222222e-1_f64) * t41359 + F::cast_from(0.71030123456790123454e-1_f64) * t41361 + F::cast_from(0.9132444444444444444e-1_f64) * t41363 - F::cast_from(0.13698666666666666667e0_f64) * t41365 + F::cast_from(0.4566222222222222222e-1_f64) * t41367 - F::cast_from(0.9132444444444444444e-1_f64) * t41369;
    t41926
}
