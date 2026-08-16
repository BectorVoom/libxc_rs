//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1010/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1010(t2101: f64, t2133: f64, t8392: f64, t9420: f64, t597: f64, t9132: f64, t9141: f64, t1882: f64, t9286: f64, t2192: f64, t8232: f64, t1901: f64, t2142: f64, t2190: f64, t2210: f64, t2221: f64, t2222: f64, t2224: f64, t379: f64, t446: f64, t558: f64, t574: f64, t604: f64, t605: f64, t7745: f64, t9135: f64, t9258: f64, t9284: f64, t9428: f64) -> f64 {
    let t41198 = t2101 * t2133;
    let t41207 = t8392 * t9420;
    let t41209 = t9132 * t597;
    let t41213 = t8392 * t9141;
    let t41220 = t1882 * t9286;
    let t41235 = t8232 * t2192;
    let t41237 = 4.0_f64 / 3.0_f64 * t1901 * t41198 * t2224 + 4.0_f64 / 9.0_f64 * t1901 * t2221 * t2222 * t7745 * t558 - 8.0_f64 / 9.0_f64 * t41207 - 8.0_f64 / 3.0_f64 * t1901 * t41209 * t9135 + 8.0_f64 / 9.0_f64 * t41213 + 4.0_f64 / 9.0_f64 * t1901 * t2210 * t604 * t9258 * t379 - 4.0_f64 / 3.0_f64 * t41220 + 4.0_f64 * t446 * t574 * t2142 * t9284 + 4.0_f64 / 3.0_f64 * t446 * t574 * t605 * t9258 * t558 + 4.0_f64 * t446 * t574 * t9428 * t2190 + 16.0_f64 / 9.0_f64 * t41235;
    t41237
}
