//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1266/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1266(t2531: f64, t2536: f64, t2539: f64, t2549: f64, t2557: f64, t2591: f64, t2598: f64, t2601: f64, t2605: f64, t268: f64, t675: f64, t724: f64, t731: f64, t746: f64, t9278: f64, t9308: f64, t9316: f64, t9329: f64, t9333: f64, t9433: f64, t9435: f64, t9447: f64, t9450: f64, t9454: f64, t9461: f64, t9469: f64, t9476: f64, t9480: f64, t9481: f64) -> f64 {
    let t9484 = -0.19298375398431042081e3_f64 * t9433 * t9435 + 1.0_f64 * t724 * t9447 + t9278 - t9308 - t9316 - t9329 - t9333 + 0.32530743900905219526e-1_f64 * t268 * t9450 * t2598 + 0.68493333333333333332e-1_f64 * t268 * t9454 * t731 - 0.51369999999999999999e-1_f64 * t268 * t2531 * t2549 - 0.16522625736956710527e1_f64 * t268 * t9461 * t2557 + 0.10274e0_f64 * t268 * t675 * t2536 * t2539 + 0.21687162600603479684e-1_f64 * t268 * t9469 * t746 - 0.16265371950452609763e-1_f64 * t268 * t2591 * t2601 - 0.48159733137676571078e0_f64 * t268 * t9476 * t2605 - 0.10389515463408878255e3_f64 * t9480 * t9481;
    t9484
}
