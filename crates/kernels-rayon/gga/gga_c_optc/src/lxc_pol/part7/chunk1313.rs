//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1313/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1313(t26314: f64, t26319: f64, t26339: f64, t26343: f64, t26363: f64, t26365: f64, t26367: f64, t26369: f64, t26372: f64, t26376: f64, t26379: f64, t26382: f64, t26385: f64, t26388: f64) -> f64 {
    let t26391 = -0.88582716049382716048e0_f64 * t26339 - 0.29896666666666666667e0_f64 * t26343 - 0.54771111111111111111e0_f64 * t26363 - 0.18257037037037037037e0_f64 * t26365 + 0.21908444444444444444e0_f64 * t26367 + 0.97370864197530864196e-1_f64 * t26369 + 0.39862222222222222223e0_f64 * t26314 + 0.10954222222222222222e1_f64 * t26372 - 0.85199506172839506175e-1_f64 * t26376 - 0.82156666666666666667e-1_f64 * t26379 - 0.21908444444444444444e0_f64 * t26382 + 0.65725333333333333332e0_f64 * t26385 - 0.10954222222222222222e0_f64 * t26388 + 0.23917333333333333333e1_f64 * t26319;
    t26391
}
