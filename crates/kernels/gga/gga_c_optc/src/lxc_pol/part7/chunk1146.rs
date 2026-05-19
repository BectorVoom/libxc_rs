//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1146/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1146<F: Float>(t23859: F, t23872: F, t787: F, t23543: F, t23545: F, t23551: F, t23553: F, t23555: F, t23557: F, t23561: F, t23565: F, t23567: F, t23569: F, t23840: F, t23842: F, t23846: F) -> (F, F, F) {
    let t23873 = t23859 + t23872;
    let t23874 = t787 * t23873;
    let t23882 = -F::cast_from(0.18396666666666666667e0_f64) * t23543 - F::new(0.44152e0) * t23545 + F::new(0.44152e0) * t23551 + F::cast_from(0.98115555555555555556e0_f64) * t23553 + F::cast_from(0.247573125e0_f64) * t23840 - F::new(0.3883875e1) * t23842 + F::cast_from(0.6189328125e-1_f64) * t23846 + F::new(0.16504875e0) * t23874 + F::new(0.11038e1) * t23555 + F::new(0.132456e1) * t23557 - F::new(0.99342e0) * t23561 - F::new(0.82785e-1) * t23565 + F::new(0.22076e0) * t23567 + F::cast_from(0.98115555555555555555e-1_f64) * t23569;
    (t23873, t23874, t23882)
}
