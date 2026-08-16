//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1429/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1429(t1992: f64, t22635: f64, t26990: f64, t115332: f64, t1985: f64, t7700: f64, t120350: f64, t120375: f64, t113967: f64, t113988: f64, t114000: f64, t115447: f64, t120342: f64, t120344: f64, t120348: f64, t120357: f64, t120363: f64, t120366: f64, t120369: f64, t120372: f64, t120377: f64, t120379: f64, t120381: f64, t120383: f64) -> (f64, f64, f64) {
    let t122399 = t1992 * t22635 * t26990;
    let t122406 = t1985 * t115332 * t7700;
    let t122411 = 7.0_f64 / 1152.0_f64 * t120350;
    let t122417 = 7.0_f64 / 288.0_f64 * t120375;
    let t122423 = -t120342 / 768.0_f64 - t120344 / 768.0_f64 - t120348 / 768.0_f64 + t122411 + 5.0_f64 / 192.0_f64 * t120357 + t113967 + 0.26915170729426927235e-3_f64 * t120363 - t115447 + 0.96894614625936938046e-2_f64 * t120366 + 0.96894614625936938046e-2_f64 * t120369 - 0.16149102437656156341e-2_f64 * t120372 + t113988 + t122417 - t120377 / 192.0_f64 - t120379 / 192.0_f64 - t120381 / 192.0_f64 + 0.67826230238155856632e-1_f64 * t120383 + 0.67826230238155856634e-1_f64 * t114000;
    (t122399, t122406, t122423)
}
