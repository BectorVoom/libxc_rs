//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1445/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1445(t104007: f64, t104009: f64, t104048: f64, t104050: f64, t22158: f64, t22162: f64, t22208: f64, t22218: f64, t22258: f64, t24741: f64, t27604: f64, t27617: f64, t6192: f64, t6203: f64, t6207: f64, t6227: f64, t6232: f64, t7345: f64, t95270: f64, t95273: f64, t95566: f64, t95623: f64, t95627: f64) -> f64 {
    let t109461 = -5.0_f64 / 2592.0_f64 * t7345 * t22208 - 5.0_f64 / 432.0_f64 * t27604 * t6203 - t27617 * t6207 / 768.0_f64 - t7345 * t22258 / 384.0_f64 + t104007 / 108.0_f64 - t7345 * t22218 / 384.0_f64 - t95623 * t6227 / 48.0_f64 + t95627 * t6232 / 96.0_f64 - t104009 / 768.0_f64 + t95566 * t6192 / 72.0_f64 + 5.0_f64 / 2304.0_f64 * t24741 * t22158 - t24741 * t22162 / 768.0_f64 + t95270 * t6227 / 256.0_f64 - t95273 * t6232 / 512.0_f64 + t104048 / 768.0_f64 - t104050 / 72.0_f64;
    t109461
}
