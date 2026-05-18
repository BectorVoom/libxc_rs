//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 799/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk799<F: Float>(t2531: F, t2536: F, t2539: F, t2549: F, t2557: F, t2591: F, t2598: F, t2601: F, t2605: F, t268: F, t675: F, t724: F, t731: F, t746: F, t9278: F, t9308: F, t9316: F, t9329: F, t9333: F, t9433: F, t9435: F, t9447: F, t9450: F, t9454: F, t9461: F, t9469: F, t9476: F, t9480: F, t9481: F) -> F {
    let t9484 = -F::new(0.19298375398431042081e3) * t9433 * t9435 + F::new(1.0) * t724 * t9447 + t9278 - t9308 - t9316 - t9329 - t9333 + F::new(0.32530743900905219526e-1) * t268 * t9450 * t2598 + F::new(0.68493333333333333332e-1) * t268 * t9454 * t731 - F::new(0.51369999999999999999e-1) * t268 * t2531 * t2549 - F::new(0.16522625736956710527e1) * t268 * t9461 * t2557 + F::new(0.10274e0) * t268 * t675 * t2536 * t2539 + F::new(0.21687162600603479684e-1) * t268 * t9469 * t746 - F::new(0.16265371950452609763e-1) * t268 * t2591 * t2601 - F::new(0.48159733137676571078e0) * t268 * t9476 * t2605 - F::new(0.10389515463408878255e3) * t9480 * t9481;
    t9484
}
