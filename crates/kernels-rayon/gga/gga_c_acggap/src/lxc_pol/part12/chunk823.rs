//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 823/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk823(t8650: f64, t8632: f64, t8635: f64, t8638: f64, t8640: f64, t8642: f64, t8644: f64, t8646: f64, t8654: f64, t8658: f64, t8662: f64, t8666: f64, t8668: f64, t8670: f64, t8672: f64) -> f64 {
    let t9239 = 0.10718504529517434243e-2_f64 * t8650;
    let t9247 = 0.1528125e-1_f64 * t8632 + t8635 / 16.0_f64 + t8638 / 64.0_f64 + 0.34299214494455789578e-2_f64 * t8640 - 0.17149607247227894789e-2_f64 * t8642 + 0.17149607247227894789e-2_f64 * t8644 - 0.85748036236139473944e-3_f64 * t8646 + t9239 + 0.64311027177104605458e-2_f64 * t8654 - 0.94344276868812456207e-3_f64 * t8658 - 0.47172138434406228104e-2_f64 * t8662 + 0.20965394859736101379e-3_f64 * t8666 + 0.34299214494455789578e-2_f64 * t8668 - 0.17149607247227894789e-2_f64 * t8670 + 0.17149607247227894789e-2_f64 * t8672;
    t9247
}
