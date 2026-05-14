//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 901/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk901<F: Float>(t2101: F, t2133: F, t8392: F, t9420: F, t597: F, t9132: F, t9141: F, t1882: F, t9286: F, t2192: F, t8232: F, t1901: F, t2142: F, t2190: F, t2210: F, t2221: F, t2222: F, t2224: F, t379: F, t446: F, t558: F, t574: F, t604: F, t605: F, t7745: F, t9135: F, t9258: F, t9284: F, t9428: F) -> (F,) {
    let t41198 = t2101 * t2133;
    let t41207 = t8392 * t9420;
    let t41209 = t9132 * t597;
    let t41213 = t8392 * t9141;
    let t41220 = t1882 * t9286;
    let t41235 = t8232 * t2192;
    let t41237 = 4.0 / 3.0 * t1901 * t41198 * t2224 + 4.0 / 9.0 * t1901 * t2221 * t2222 * t7745 * t558 - 8.0 / 9.0 * t41207 - 8.0 / 3.0 * t1901 * t41209 * t9135 + 8.0 / 9.0 * t41213 + 4.0 / 9.0 * t1901 * t2210 * t604 * t9258 * t379 - 4.0 / 3.0 * t41220 + 4.0 * t446 * t574 * t2142 * t9284 + 4.0 / 3.0 * t446 * t574 * t605 * t9258 * t558 + 4.0 * t446 * t574 * t9428 * t2190 + 16.0 / 9.0 * t41235;
    (t41237,)
}
