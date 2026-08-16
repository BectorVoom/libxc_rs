//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1334/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1334(t49385: f64, t49387: f64, t49393: f64, t49395: f64, t57012: f64, t57016: f64, t57020: f64, t57024: f64, t57027: f64, t57030: f64, t57034: f64, t39545: f64, t39560: f64, t39565: f64, t49404: f64, t49406: f64, t57037: f64, t57041: f64, t57044: f64, t57048: f64, t57057: f64, t57060: f64, t57063: f64) -> (f64, f64) {
    let t58080 = -0.10340444444444444444e2_f64 * t49385 + 0.15510666666666666667e2_f64 * t49387 + 0.25851111111111111111e1_f64 * t49393 + 0.28723456790123456789e1_f64 * t49395 + 0.29556e-1_f64 * t57012 + 0.7389e-2_f64 * t57016 - 0.12315e-2_f64 * t57020 - 0.46531999999999999999e2_f64 * t57024 - 0.38776666666666666665e1_f64 * t57027 - 0.14778e-1_f64 * t57030 + 0.6568e-2_f64 * t57034;
    let t58093 = 0.2585111111111111111e2_f64 * t57037 - 0.57446913580246913579e1_f64 * t57041 - 0.12771111111111111111e-2_f64 * t57044 - 0.12315e-2_f64 * t57048 - 0.27366666666666666666e-2_f64 * t39545 - 0.821e-2_f64 * t39560 + 0.1642e-1_f64 * t39565 + 0.19704e-1_f64 * t49404 - 0.6568e-2_f64 * t49406 - 0.19388333333333333333e1_f64 * t57057 + 0.46532e2_f64 * t57060 + 0.11633e2_f64 * t57063;
    (t58080, t58093)
}
