//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1296/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1296<F: Float>(t39411: F, t39413: F, t39418: F, t49240: F, t49242: F, t49271: F, t49273: F, t56966: F, t56969: F, t56972: F, t56975: F, t56978: F, t56981: F, t56984: F) -> F {
    let t57135 = -F::cast_from(0.80513333333333333336e0_f64) * t39411 - F::cast_from(0.53675555555555555556e0_f64) * t39413 + F::cast_from(0.16102666666666666667e1_f64) * t39418 + F::cast_from(0.80513333333333333333e0_f64) * t49240 - F::new(0.24154e1) * t49242 - F::new(0.132456e1) * t49271 + F::new(0.22076e0) * t49273 + F::new(0.72462e1) * t56966 - F::cast_from(0.20128333333333333334e1_f64) * t56969 - F::new(0.11038e0) * t56972 - F::new(0.22076e0) * t56975 - F::new(0.108693e2) * t56978 + F::new(0.24154e1) * t56981 - F::cast_from(0.80513333333333333332e0_f64) * t56984;
    t57135
}
