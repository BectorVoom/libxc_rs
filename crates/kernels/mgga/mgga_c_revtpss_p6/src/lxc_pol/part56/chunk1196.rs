//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1196/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1196<F: Float>(t127346: F, t127349: F, t127357: F, t127359: F, t127361: F, t127366: F, t127369: F, t127371: F, t127373: F, t127375: F, t127378: F, t127384: F, t127385: F, t129468: F, t129471: F, t129473: F, t1453: F, t29427: F, t33346: F, t34880: F, t4293: F, t7591: F) -> F {
    let t132116 = t1453 * t34880 - F::new(4.0) * t29427 * t7591 - F::new(2.0) * t33346 * t4293 - t127346 + t127349 - t127357 - t127359 + t127361 - t127366 - t127369 - t127371 - t127373 - t127375 - t127378 - t127384 - t127385 - F::new(4.0) * t129468 - F::new(4.0) * t129471 - F::new(4.0) * t129473;
    t132116
}
