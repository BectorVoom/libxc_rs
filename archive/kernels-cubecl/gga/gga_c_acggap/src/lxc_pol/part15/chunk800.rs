//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 800/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk800<F: Float>(t8650: F, t8632: F, t8635: F, t8638: F, t8640: F, t8642: F, t8644: F, t8646: F, t8654: F, t8658: F, t8662: F, t8666: F, t8668: F, t8670: F, t8672: F) -> F {
    let t9239 = F::cast_from(0.10718504529517434243e-2_f64) * t8650;
    let t9247 = F::cast_from(0.1528125e-1_f64) * t8632 + t8635 / F::cast_from(16.0_f64) + t8638 / F::cast_from(64.0_f64) + F::cast_from(0.34299214494455789578e-2_f64) * t8640 - F::cast_from(0.17149607247227894789e-2_f64) * t8642 + F::cast_from(0.17149607247227894789e-2_f64) * t8644 - F::cast_from(0.85748036236139473944e-3_f64) * t8646 + t9239 + F::cast_from(0.64311027177104605458e-2_f64) * t8654 - F::cast_from(0.94344276868812456207e-3_f64) * t8658 - F::cast_from(0.47172138434406228104e-2_f64) * t8662 + F::cast_from(0.20965394859736101379e-3_f64) * t8666 + F::cast_from(0.34299214494455789578e-2_f64) * t8668 - F::cast_from(0.17149607247227894789e-2_f64) * t8670 + F::cast_from(0.17149607247227894789e-2_f64) * t8672;
    t9247
}
