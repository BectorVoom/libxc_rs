//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 743/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk743<F: Float>(t7649: F, t7651: F, t7653: F, t7655: F, t8232: F, t8801: F, t8804: F, t8808: F, t8811: F, t8814: F, t8818: F, t8821: F, t8824: F, t8827: F, t9309: F, t8841: F) -> (F, F) {
    let t9310 = t7649 + t8801 / 64.0 + t8804 / 96.0 + t8808 / 8.0 + t8811 / 24.0 + 0.22921875e-1 * t8814 + 0.22921875e-1 * t8818 + 0.1528125e-1 * t8821 + 0.22921875e-1 * t8824 + 0.1528125e-1 * t8827 - t9309 + t7651 - t7653 + t7655 + t8232;
    let t9313 = 0.17149607247227894789e-2 * t8841;
    (t9310, t9313)
}
