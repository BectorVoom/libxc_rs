//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2685/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2685(t1352: f64, t16224: f64, t16306: f64, t20448: f64, t20563: f64, t3803: f64, t54556: f64, t54582: f64, t54612: f64, t57308: f64, t57310: f64, t57324: f64, t57383: f64, t57392: f64, t57396: f64, t57398: f64, t57407: f64, t57409: f64) -> f64 {
    let t74806 = -5.0_f64 / 256.0_f64 * t3803 * t16224 * t20563 * t1352 + 7.0_f64 / 1536.0_f64 * t57308 - 119.0_f64 / 4608.0_f64 * t57310 - t54556 - 7.0_f64 / 1536.0_f64 * t57324 + 119.0_f64 / 4608.0_f64 * t57383 + 455.0_f64 / 216.0_f64 * t54582 + 7.0_f64 / 768.0_f64 * t57392 + 35.0_f64 / 64.0_f64 * t57396 - 35.0_f64 / 192.0_f64 * t57398 + 7.0_f64 / 1536.0_f64 * t57407 + 7.0_f64 / 1536.0_f64 * t57409 - 5.0_f64 / 256.0_f64 * t3803 * t16224 * t16306 * t20448 + t54612;
    t74806
}
