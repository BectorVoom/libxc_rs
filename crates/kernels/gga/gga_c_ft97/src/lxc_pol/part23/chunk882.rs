//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 882/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk882<F: Float>(t27742: F, t743: F, t1434: F, t193: F, t10157: F, t3837: F, t6119: F, t6118: F, t3875: F, t6135: F, t24432: F, t24531: F, t3886: F, t24438: F, t24455: F, t24470: F, t27466: F, t27471: F, t27473: F, t27477: F, t27481: F, t27485: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t27743 = t743 * t27742;
    let t27745 = t1434 * t193 * t27743;
    let t27750 = t10157 * t6119 * t3837;
    let t27751 = t6118 * t27750;
    let t27753 = t6135 * t3875;
    let t27754 = t24432 * t27753;
    let t27755 = t6118 * t27754;
    let t27757 = t24531 * t3886;
    let t27758 = t24438 * t27757;
    let t27759 = t6118 * t27758;
    let t27761 = t27466 / 6.0 + t27471 / 3.0 - t27473 / 9.0 - 2.0 / 3.0 * t27477 - 6.0 * t27481 + t27485 / 3.0 - t27745 / 2.0 - t24455 / 12.0 - t24470 / 3.0 - 3.0 * t27751 - t27755 / 3.0 - t27759 / 3.0;
    (t27743, t27745, t27750, t27751, t27753, t27754, t27755, t27757, t27758, t27759, t27761)
}
