//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1077/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1077(t10431: f64, t139: f64, t214: f64, t26: f64, t1318: f64, t3272: f64, t10182: f64, t10185: f64, t10188: f64, t10191: f64, t10195: f64, t10200: f64, t10205: f64, t10212: f64, t136: f64, t3138: f64, t3140: f64, t3142: f64, t3947: f64, t677: f64, t8511: f64, t8513: f64, t8519: f64, t8526: f64, t8534: f64, t8547: f64) -> (f64, f64, f64, f64, f64) {
    let t10432 = t139 * t10431;
    let t10433 = t10432 * t214;
    let t10434 = t26 * t10433;
    let t10437 = t3272 * t1318;
    let t10438 = t26 * t10437;
    let t10441 = -t10182 / 192.0_f64 - t10185 / 96.0_f64 - t10188 / 96.0_f64 - t3138 * t3140 * t10191 / 48.0_f64 - t8534 - t8547 - t3138 * t10195 * t3142 / 24.0_f64 - t3138 * t3140 * t10200 / 24.0_f64 + t8526 * t3140 * t10205 / 16.0_f64 - 7.0_f64 / 144.0_f64 * t8511 * t8513 * t10205 + t3138 * t8519 * t10212 / 12.0_f64 - 3.0_f64 / 32.0_f64 * t677 * t3947 - 3.0_f64 / 64.0_f64 * t136 * t10434 - 3.0_f64 / 32.0_f64 * t136 * t10438;
    (t10433, t10434, t10437, t10438, t10441)
}
