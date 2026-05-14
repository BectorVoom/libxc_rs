//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 727/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk727<F: Float>(t7648: F, t7650: F, t7652: F, t7654: F, t7672: F, t8801: F, t8804: F, t8808: F, t8811: F, t8814: F, t8818: F, t8821: F, t8824: F, t8827: F, t8829: F, t527: F, t7685: F) -> (F, F) {
    let t8834 = 0.42874018118069736972e-3 * t7648 + t8801 / 128.0 + t8804 / 192.0 + t8808 / 16.0 + t8811 / 48.0 + 0.114609375e-1 * t8814 + 0.114609375e-1 * t8818 + 0.7640625e-2 * t8821 + 0.114609375e-1 * t8824 + 0.7640625e-2 * t8827 - 0.420234375e-1 * t8829 + 0.17149607247227894789e-2 * t7650 - 0.85748036236139473944e-3 * t7652 + 0.85748036236139473944e-3 * t7654 + t7672;
    let t8835 = t7685 * t527;
    (t8834, t8835)
}
