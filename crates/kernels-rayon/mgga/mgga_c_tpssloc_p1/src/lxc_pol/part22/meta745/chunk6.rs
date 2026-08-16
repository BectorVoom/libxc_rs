//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2479/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2479(t135: f64, t21561: f64, t973: f64, t10390: f64, t10413: f64, t14207: f64, t17712: f64, t17732: f64, t17984: f64, t21526: f64, t21566: f64, t3071: f64, t3130: f64, t369: f64, t378: f64, t42505: f64, t4347: f64, t4582: f64, t50265: f64, t5869: f64, t5878: f64, t62164: f64, t62177: f64, t62183: f64, t68: f64, t70012: f64) -> f64 {
    let t70497 = t973 * t135 * t21561;
    let t70509 = t3130 * t4582 * t17712 * t17732 / 512.0_f64 - 3.0_f64 / 512.0_f64 * t50265 * t17984 - t62164 / 1536.0_f64 - t62177 / 4608.0_f64 + t62183 / 4608.0_f64 - t42505 * t21526 / 144.0_f64 + t14207 * t5869 / 1024.0_f64 + t70497 / 144.0_f64 + t70012 * t68 * t369 * t378 / 3072.0_f64 - t10413 * t3071 * t5878 * t4347 / 1536.0_f64 + t10390 * t21566 / 1536.0_f64;
    t70509
}
