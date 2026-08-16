//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1333/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1333<F: Float>(t39411: F, t39413: F, t39418: F, t49240: F, t49242: F, t49271: F, t49273: F, t56966: F, t56969: F, t56972: F, t56975: F, t23683: F, t23686: F, t30189: F, t30270: F, t49378: F, t49381: F, t56978: F, t56981: F, t56984: F, t56988: F, t56991: F, t56994: F) -> (F, F) {
    let t58056 = -F::cast_from(0.51702222222222222221e1_f64) * t39411 - F::cast_from(0.34468148148148148146e1_f64) * t39413 + F::cast_from(0.10340444444444444444e2_f64) * t39418 + F::cast_from(0.5170222222222222222e1_f64) * t49240 - F::cast_from(0.15510666666666666667e2_f64) * t49242 - F::cast_from(0.19704e-1_f64) * t49271 + F::cast_from(0.3284e-2_f64) * t49273 + F::cast_from(0.46531999999999999998e2_f64) * t56966 - F::cast_from(0.12925555555555555555e2_f64) * t56969 - F::cast_from(0.1642e-2_f64) * t56972 - F::cast_from(0.3284e-2_f64) * t56975;
    let t58067 = -F::cast_from(0.69798e2_f64) * t56978 + F::cast_from(0.15510666666666666667e2_f64) * t56981 - F::cast_from(0.5170222222222222222e1_f64) * t56984 - F::cast_from(0.44334e-1_f64) * t56988 + F::cast_from(0.9852e-2_f64) * t56991 + F::cast_from(0.14778e-1_f64) * t56994 + F::cast_from(0.14595555555555555556e-1_f64) * t30189 + t23683 + t23686 + F::cast_from(0.14595555555555555556e-2_f64) * t49378 + F::cast_from(0.3284e-2_f64) * t49381 + F::cast_from(0.8042567901234567901e1_f64) * t30270;
    (t58056, t58067)
}
