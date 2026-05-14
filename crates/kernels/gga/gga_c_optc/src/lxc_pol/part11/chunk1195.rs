//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1195/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1195<F: Float>(t49385: F, t49387: F, t49393: F, t49395: F, t57012: F, t57016: F, t57020: F, t57024: F, t57027: F, t57030: F, t57034: F, t39545: F, t39560: F, t39565: F, t49404: F, t49406: F, t57037: F, t57041: F, t57044: F, t57048: F, t57057: F, t57060: F, t57063: F) -> (F, F) {
    let t58080 = -0.10340444444444444444e2 * t49385 + 0.15510666666666666667e2 * t49387 + 0.25851111111111111111e1 * t49393 + 0.28723456790123456789e1 * t49395 + 0.29556e-1 * t57012 + 0.7389e-2 * t57016 - 0.12315e-2 * t57020 - 0.46531999999999999999e2 * t57024 - 0.38776666666666666665e1 * t57027 - 0.14778e-1 * t57030 + 0.6568e-2 * t57034;
    let t58093 = 0.2585111111111111111e2 * t57037 - 0.57446913580246913579e1 * t57041 - 0.12771111111111111111e-2 * t57044 - 0.12315e-2 * t57048 - 0.27366666666666666666e-2 * t39545 - 0.821e-2 * t39560 + 0.1642e-1 * t39565 + 0.19704e-1 * t49404 - 0.6568e-2 * t49406 - 0.19388333333333333333e1 * t57057 + 0.46532e2 * t57060 + 0.11633e2 * t57063;
    (t58080, t58093)
}
