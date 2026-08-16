//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1166/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1166(t11434: f64, t26331: f64, t5544: f64, t21991: f64, t3021: f64, t34378: f64, t34380: f64, t34383: f64, t34386: f64, t34388: f64, t34390: f64, t34394: f64, t34397: f64, t34400: f64) -> f64 {
    let t34403 = t11434 * t26331 * t5544;
    let t34406 = t11434 * t3021 * t21991;
    let t34408 = 0.17376185052903442709e-3_f64 * t34378 + 0.22510123728325872388e-7_f64 * t34380 + 0.22510123728325872388e-6_f64 * t34383 + 0.22510123728325872388e-7_f64 * t34386 + 0.22510123728325872388e-6_f64 * t34388 - 0.11594181388521408695e-4_f64 * t34390 - 0.34752370105806885418e-3_f64 * t34394 - 0.20611878024038059902e-5_f64 * t34397 - 0.21720231316129303386e-4_f64 * t34400 - 0.21720231316129303386e-4_f64 * t34403 - 0.10860115658064651693e-4_f64 * t34406;
    t34408
}
