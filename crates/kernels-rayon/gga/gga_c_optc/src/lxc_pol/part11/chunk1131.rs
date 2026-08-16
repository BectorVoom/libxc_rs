//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1131/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1131(t16513: f64, t2144: f64, t16496: f64, t2164: f64, t16557: f64, t7122: f64, t16471: f64, t7018: f64, t16479: f64, t23013: f64, t16464: f64, t13611: f64, t4054: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t49070 = t2144 * t16513;
    let t49072 = t2164 * t16496;
    let t49106 = t7122 * t16557;
    let t49142 = t7018 * t16471;
    let t49144 = t23013 * t16479;
    let t49172 = t2144 * t16464;
    let t49197 = t4054 * t13611;
    (t49070, t49072, t49106, t49142, t49144, t49172, t49197)
}
