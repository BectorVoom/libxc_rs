//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2262/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2262<F: Float>(t101124: F, t101416: F, t101420: F, t101422: F, t101428: F, t101431: F, t101436: F, t104163: F, t104408: F, t1310: F, t29422: F, t508: F, t98603: F, t98605: F, t98607: F, t98609: F, t98611: F, t98615: F, t98617: F, t98621: F, t98623: F) -> F {
    let t105741 = -F::cast_from(2.0_f64) * t104163 * t508 - t104408 * t508 - F::cast_from(2.0_f64) * t1310 * t29422 - t101124 + t101416 + t101420 + t101422 + t101428 + t101431 + t101436 - t98603 - t98605 - t98607 - t98609 - t98611 - t98615 - t98617 + t98621 - t98623;
    t105741
}
