//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1012/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1012<F: Float>(t11496: F, t185: F, t9386: F, t11435: F, t129: F, t21778: F, t11434: F, t26331: F, t5544: F, t21991: F, t3021: F, t34378: F, t34380: F, t34383: F, t34386: F, t34388: F, t34390: F, t34394: F) -> (F,) {
    let t34397 = t185 * t9386 * t11496;
    let t34400 = t21778 * t129 * t11435;
    let t34403 = t11434 * t26331 * t5544;
    let t34406 = t11434 * t3021 * t21991;
    let t34408 = 0.17376185052903442709e-3 * t34378 + 0.22510123728325872388e-7 * t34380 + 0.22510123728325872388e-6 * t34383 + 0.22510123728325872388e-7 * t34386 + 0.22510123728325872388e-6 * t34388 - 0.11594181388521408695e-4 * t34390 - 0.34752370105806885418e-3 * t34394 - 0.20611878024038059902e-5 * t34397 - 0.21720231316129303386e-4 * t34400 - 0.21720231316129303386e-4 * t34403 - 0.10860115658064651693e-4 * t34406;
    (t34408,)
}
