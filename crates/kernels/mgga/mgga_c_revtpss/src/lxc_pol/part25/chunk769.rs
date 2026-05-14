//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 769/1212 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk769<F: Float>(t730: F, t9446: F, t2596: F, t675: F, t215: F, t723: F, t2553: F, t738: F, t2491: F, t177: F, t9417: F, t2495: F, t9368: F, t2531: F, t2536: F, t2539: F, t2549: F, t2557: F, t2591: F, t2598: F, t2601: F, t2605: F, t268: F, t724: F, t731: F, t746: F, t9278: F, t9308: F, t9316: F, t9329: F, t9333: F, t9433: F, t9435: F) -> (F,) {
    let t9447 = t9446 * t730;
    let t9450 = t675 * t2596;
    let t9454 = t215 * t723;
    let t9461 = t675 * t2553;
    let t9469 = t215 * t738;
    let t9476 = t675 * t2491;
    let t9480 = t177 * t9417;
    let t9481 = t9368 * t2495;
    let t9484 = -0.19298375398431042081e3 * t9433 * t9435 + 1.0 * t724 * t9447 + t9278 - t9308 - t9316 - t9329 - t9333 + 0.32530743900905219526e-1 * t268 * t9450 * t2598 + 0.68493333333333333332e-1 * t268 * t9454 * t731 - 0.51369999999999999999e-1 * t268 * t2531 * t2549 - 0.16522625736956710527e1 * t268 * t9461 * t2557 + 0.10274e0 * t268 * t675 * t2536 * t2539 + 0.21687162600603479684e-1 * t268 * t9469 * t746 - 0.16265371950452609763e-1 * t268 * t2591 * t2601 - 0.48159733137676571078e0 * t268 * t9476 * t2605 - 0.10389515463408878255e3 * t9480 * t9481;
    (t9484,)
}
