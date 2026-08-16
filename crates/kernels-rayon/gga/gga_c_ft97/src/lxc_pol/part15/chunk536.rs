//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 536/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk536(t2882: f64, t5413: f64, t2881: f64, t319: f64, t4969: f64, t835: f64, t1901: f64, t193: f64, t2816: f64, t4156: f64, t4271: f64, t4273: f64, t4283: f64, t446: f64, t5311: f64, t5315: f64, t5319: f64, t5323: f64, t5327: f64, t5332: f64, t5376: f64, t5381: f64, t5395: f64, t5399: f64, t5403: f64, t5410: f64, t89: f64) -> (f64, f64, f64, f64) {
    let t5414 = t2882 * t5413;
    let t5415 = t2881 * t5414;
    let t5419 = t835 * t319 * t4969;
    let t5422 = t2816 + 2.0_f64 / 3.0_f64 * t446 * t5311 - 2.0_f64 / 9.0_f64 * t446 * t5315 - t446 * t5319 / 9.0_f64 - 2.0_f64 / 27.0_f64 * t446 * t5323 + 2.0_f64 / 3.0_f64 * t446 * t5327 + 2.0_f64 / 3.0_f64 * t446 * t5332 + 2.0_f64 / 9.0_f64 * t4156 + 2.0_f64 / 9.0_f64 * t4273 + t89 * t193 * t5376 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t5381 - t446 * t5395 / 3.0_f64 - t446 * t5399 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t5403 - 2.0_f64 / 9.0_f64 * t4271 + 2.0_f64 / 27.0_f64 * t4283 + 2.0_f64 / 9.0_f64 * t1901 * t5410 + 2.0_f64 / 9.0_f64 * t1901 * t5415 + 2.0_f64 / 9.0_f64 * t446 * t5419;
    (t5414, t5415, t5419, t5422)
}
