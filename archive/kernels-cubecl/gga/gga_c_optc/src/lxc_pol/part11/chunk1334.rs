//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1334/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1334<F: Float>(t49385: F, t49387: F, t49393: F, t49395: F, t57012: F, t57016: F, t57020: F, t57024: F, t57027: F, t57030: F, t57034: F, t39545: F, t39560: F, t39565: F, t49404: F, t49406: F, t57037: F, t57041: F, t57044: F, t57048: F, t57057: F, t57060: F, t57063: F) -> (F, F) {
    let t58080 = -F::cast_from(0.10340444444444444444e2_f64) * t49385 + F::cast_from(0.15510666666666666667e2_f64) * t49387 + F::cast_from(0.25851111111111111111e1_f64) * t49393 + F::cast_from(0.28723456790123456789e1_f64) * t49395 + F::cast_from(0.29556e-1_f64) * t57012 + F::cast_from(0.7389e-2_f64) * t57016 - F::cast_from(0.12315e-2_f64) * t57020 - F::cast_from(0.46531999999999999999e2_f64) * t57024 - F::cast_from(0.38776666666666666665e1_f64) * t57027 - F::cast_from(0.14778e-1_f64) * t57030 + F::cast_from(0.6568e-2_f64) * t57034;
    let t58093 = F::cast_from(0.2585111111111111111e2_f64) * t57037 - F::cast_from(0.57446913580246913579e1_f64) * t57041 - F::cast_from(0.12771111111111111111e-2_f64) * t57044 - F::cast_from(0.12315e-2_f64) * t57048 - F::cast_from(0.27366666666666666666e-2_f64) * t39545 - F::cast_from(0.821e-2_f64) * t39560 + F::cast_from(0.1642e-1_f64) * t39565 + F::cast_from(0.19704e-1_f64) * t49404 - F::cast_from(0.6568e-2_f64) * t49406 - F::cast_from(0.19388333333333333333e1_f64) * t57057 + F::cast_from(0.46532e2_f64) * t57060 + F::cast_from(0.11633e2_f64) * t57063;
    (t58080, t58093)
}
