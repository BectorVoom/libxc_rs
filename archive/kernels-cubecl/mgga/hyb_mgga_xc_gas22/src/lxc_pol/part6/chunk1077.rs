//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1077/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1077<F: Float>(t10431: F, t139: F, t214: F, t26: F, t1318: F, t3272: F, t10182: F, t10185: F, t10188: F, t10191: F, t10195: F, t10200: F, t10205: F, t10212: F, t136: F, t3138: F, t3140: F, t3142: F, t3947: F, t677: F, t8511: F, t8513: F, t8519: F, t8526: F, t8534: F, t8547: F) -> (F, F, F, F, F) {
    let t10432 = t139 * t10431;
    let t10433 = t10432 * t214;
    let t10434 = t26 * t10433;
    let t10437 = t3272 * t1318;
    let t10438 = t26 * t10437;
    let t10441 = -t10182 / F::cast_from(192.0_f64) - t10185 / F::cast_from(96.0_f64) - t10188 / F::cast_from(96.0_f64) - t3138 * t3140 * t10191 / F::cast_from(48.0_f64) - t8534 - t8547 - t3138 * t10195 * t3142 / F::cast_from(24.0_f64) - t3138 * t3140 * t10200 / F::cast_from(24.0_f64) + t8526 * t3140 * t10205 / F::cast_from(16.0_f64) - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t8511 * t8513 * t10205 + t3138 * t8519 * t10212 / F::cast_from(12.0_f64) - F::cast_from(3.0_f64) / F::cast_from(32.0_f64) * t677 * t3947 - F::cast_from(3.0_f64) / F::cast_from(64.0_f64) * t136 * t10434 - F::cast_from(3.0_f64) / F::cast_from(32.0_f64) * t136 * t10438;
    (t10433, t10434, t10437, t10438, t10441)
}
