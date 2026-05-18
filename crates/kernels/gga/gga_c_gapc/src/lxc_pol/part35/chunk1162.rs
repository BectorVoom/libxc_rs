//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1162/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1162<F: Float>(t11434: F, t26331: F, t5544: F, t21991: F, t3021: F, t34378: F, t34380: F, t34383: F, t34386: F, t34388: F, t34390: F, t34394: F, t34397: F, t34400: F) -> F {
    let t34403 = t11434 * t26331 * t5544;
    let t34406 = t11434 * t3021 * t21991;
    let t34408 = F::new(0.17376185052903442709e-3) * t34378 + F::new(0.22510123728325872388e-7) * t34380 + F::new(0.22510123728325872388e-6) * t34383 + F::new(0.22510123728325872388e-7) * t34386 + F::new(0.22510123728325872388e-6) * t34388 - F::new(0.11594181388521408695e-4) * t34390 - F::new(0.34752370105806885418e-3) * t34394 - F::new(0.20611878024038059902e-5) * t34397 - F::new(0.21720231316129303386e-4) * t34400 - F::new(0.21720231316129303386e-4) * t34403 - F::new(0.10860115658064651693e-4) * t34406;
    t34408
}
