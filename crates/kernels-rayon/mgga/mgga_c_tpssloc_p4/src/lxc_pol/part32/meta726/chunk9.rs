//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2350/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2350(t2108: f64, t2240: f64, t5392: f64, t1409: f64, t605: f64, t1410: f64, t2110: f64, t24520: f64, t24526: f64, t26009: f64, t27972: f64, t27976: f64, t6492: f64, t7246: f64, t9239: f64, t96502: f64, t96506: f64, t96517: f64, t96521: f64, t96553: f64, t96556: f64) -> f64 {
    let t104907 = t2240 * t5392 * t2108;
    let t104911 = t605 * t1409 * t2108;
    let t104916 = 5.0_f64 / 3.0_f64 * t24520 * t27972 + 20.0_f64 * t9239 * t1410 * t2108 * t26009 + 5.0_f64 / 3.0_f64 * t24526 * t27972 + 5.0_f64 / 3.0_f64 * t7246 * t96502 + 5.0_f64 / 3.0_f64 * t7246 * t96506 + 5.0_f64 / 6.0_f64 * t24520 * t27976 + 5.0_f64 / 6.0_f64 * t24526 * t27976 + 5.0_f64 / 6.0_f64 * t7246 * t96517 + 5.0_f64 / 6.0_f64 * t7246 * t96521 - 5.0_f64 / 3.0_f64 * t104907 * t6492 + 2.0_f64 / 3.0_f64 * t104911 * t96553 + t96556 * t2110 / 3.0_f64;
    t104916
}
