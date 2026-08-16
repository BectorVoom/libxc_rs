//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1218/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1218(t39403: f64, t41352: f64, t41353: f64, t41354: f64, t41367: f64, t41369: f64, t41372: f64, t43009: f64, t43012: f64, t43015: f64, t43018: f64, t43021: f64) -> f64 {
    let t44188 = -t41352 - t41353 - 0.21951497276451705328e0_f64 * t43009 - t41354 - 0.17336443480108537126e0_f64 * t43012 + 0.5200933044032561138e0_f64 * t43015 + 0.17336443480108537126e0_f64 * t43018 - 0.65854491829355115984e0_f64 * t43021 + t41367 - t41369 - 0.92461031893912198008e0_f64 * t39403 + t41372;
    t44188
}
