//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1092/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1092<F: Float>(t22534: F, t22536: F, t22572: F, t37939: F, t409: F, t1728: F, t70: F, t5569: F, t5572: F, t22825: F, t22833: F, t22708: F, t22711: F, t1293: F, t1711: F, t1602: F, t92488: F) -> (F, F, F, F, F, F, F, F) {
    let t92629 = t22534 * t22572 * t22536;
    let t92642 = t37939 * t409;
    let t92652 = t1728 * t70;
    let t92654 = t5569 * t92652 * t5572;
    let t92666 = t22833 * t22825;
    let t92669 = t22708 * t22711;
    let t92685 = t1711 * t1293;
    let t92689 = t1602 * t92488;
    (t92629, t92642, t92652, t92654, t92666, t92669, t92685, t92689)
}
