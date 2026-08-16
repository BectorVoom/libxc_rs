//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1114/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1114(t10254: f64, t1882: f64, t43348: f64, t43353: f64, t43357: f64, t43361: f64, t43363: f64, t43365: f64, t43369: f64, t43373: f64, t43376: f64, t43379: f64, t43384: f64, t43388: f64, t43390: f64, t43392: f64) -> (f64, f64) {
    let t43394 = t1882 * t10254;
    let t43396 = -8.0_f64 / 27.0_f64 * t43348 - 8.0_f64 / 9.0_f64 * t43353 - 4.0_f64 / 3.0_f64 * t43357 - 16.0_f64 / 9.0_f64 * t43361 - 8.0_f64 / 27.0_f64 * t43363 - 8.0_f64 / 9.0_f64 * t43365 - 4.0_f64 / 3.0_f64 * t43369 + 8.0_f64 / 3.0_f64 * t43373 - 4.0_f64 * t43376 + 8.0_f64 / 3.0_f64 * t43379 + 8.0_f64 / 3.0_f64 * t43384 + 8.0_f64 / 3.0_f64 * t43388 + 16.0_f64 / 27.0_f64 * t43390 + 8.0_f64 / 9.0_f64 * t43392 + 8.0_f64 / 9.0_f64 * t43394;
    (t43394, t43396)
}
