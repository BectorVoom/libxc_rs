//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1131/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1131<F: Float>(t16513: F, t2144: F, t16496: F, t2164: F, t16557: F, t7122: F, t16471: F, t7018: F, t16479: F, t23013: F, t16464: F, t13611: F, t4054: F) -> (F, F, F, F, F, F, F) {
    let t49070 = t2144 * t16513;
    let t49072 = t2164 * t16496;
    let t49106 = t7122 * t16557;
    let t49142 = t7018 * t16471;
    let t49144 = t23013 * t16479;
    let t49172 = t2144 * t16464;
    let t49197 = t4054 * t13611;
    (t49070, t49072, t49106, t49142, t49144, t49172, t49197)
}
