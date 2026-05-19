//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1315/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1315<F: Float>(t4884: F, t39411: F, t49385: F, t49387: F, t56966: F, t56978: F, t56981: F, t56984: F, t57024: F, t57057: F, t57060: F, t57063: F) -> (F, F) {
    let t57453 = t4884 * t4884;
    let t57501 = -F::cast_from(0.17123333333333333333e-1_f64) * t57057 + F::new(0.41096e0) * t57060 - F::new(0.61644e0) * t56978 + F::new(0.10274e0) * t57063 - F::cast_from(0.9132444444444444444e-1_f64) * t49385 + F::cast_from(0.13698666666666666667e0_f64) * t49387 + F::cast_from(0.13698666666666666667e0_f64) * t56981 - F::cast_from(0.4566222222222222222e-1_f64) * t56984 - F::cast_from(0.45662222222222222221e-1_f64) * t39411 - F::cast_from(0.41095999999999999999e0_f64) * t57024 + F::cast_from(0.41095999999999999998e0_f64) * t56966;
    (t57453, t57501)
}
