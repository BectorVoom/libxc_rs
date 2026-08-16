//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1449/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1449(t104235: f64, t104239: f64, t1417: f64, t1932: f64, t2121: f64, t2130: f64, t2133: f64, t2136: f64, t2140: f64, t21749: f64, t22115: f64, t22154: f64, t22214: f64, t22301: f64, t22309: f64, t22314: f64, t24741: f64, t27604: f64, t27629: f64, t29594: f64, t3448: f64, t488: f64, t6169: f64, t6192: f64, t6207: f64, t6211: f64, t7345: f64, t8040: f64, t8048: f64, t86146: f64, t86164: f64, t86171: f64, t86278: f64, t95687: f64) -> f64 {
    let t109593 = -t95687 * t6192 / 384.0_f64 - t24741 * t22154 / 768.0_f64 - t2121 * t3448 * t21749 / 48.0_f64 + t86171 * t22301 / 1536.0_f64 + t27604 * t6207 / 144.0_f64 - 0.30279567070605293142e-3_f64 * t104239 * t8040 - 0.30279567070605293142e-3_f64 * t27629 * t29594 + 0.60559134141210586284e-3_f64 * t104235 * t8040 + t27604 * t6211 / 72.0_f64 - t7345 * t22214 / 2304.0_f64 + t86146 * t22309 / 256.0_f64 - t86164 * t22314 / 256.0_f64 + t86278 + t22115 * t2140 * t488 / 1536.0_f64 - t6169 * t8048 * t488 / 96.0_f64 - 0.72670960969452703541e-1_f64 / t2130 / t1417 * t1932 * t2133 * t2136;
    t109593
}
