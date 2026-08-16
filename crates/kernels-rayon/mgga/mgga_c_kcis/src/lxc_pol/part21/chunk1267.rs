//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1267/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1267(t14809: f64, t7748: f64, t95389: f64, t95392: f64, t95394: f64, t95396: f64, t95398: f64, t95400: f64, t95402: f64, t95404: f64, t95406: f64, t95408: f64, t95410: f64, t95412: f64, t95414: f64, t95417: f64, t95419: f64, t95421: f64, t95423: f64, t95425: f64, t95427: f64) -> (f64, f64) {
    let t95429 = t7748 * t14809;
    let t95431 = -t95389 / 16.0_f64 + 3.0_f64 / 64.0_f64 * t95392 - 2.0_f64 / 9.0_f64 * t95394 + 2.0_f64 / 9.0_f64 * t95396 - t95398 / 64.0_f64 - t95400 / 3.0_f64 - t95402 / 12.0_f64 - t95404 / 96.0_f64 + t95406 / 12.0_f64 + t95408 / 128.0_f64 - 19.0_f64 / 72.0_f64 * t95410 - t95412 / 64.0_f64 - 2.0_f64 / 3.0_f64 * t95414 - t95417 / 8.0_f64 - t95419 / 12.0_f64 + t95421 / 12.0_f64 + t95423 / 3.0_f64 - t95425 / 72.0_f64 - t95427 / 8.0_f64 - t95429 / 24.0_f64;
    (t95429, t95431)
}
