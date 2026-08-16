//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2740/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2740(t12470: f64, t193: f64, t3924: f64, t40224: f64, t40230: f64, t56486: f64, t57226: f64, t57228: f64, t57230: f64, t57231: f64, t57232: f64, t57233: f64, t57236: f64, t57237: f64, t6330: f64) -> f64 {
    let t57822 = 6.0_f64 * t12470 * t193 * t6330 + 12.0_f64 * t193 * t3924 * t56486 + t40224 - t40230 - t57226 + t57228 - t57230 - t57231 + t57232 + t57233 + t57236 + t57237;
    t57822
}
