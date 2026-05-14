//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1062/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1062<F: Float>(t5993: F, t7045: F, t5985: F, t7025: F, t6019: F, t7038: F, t6030: F, t25254: F, t25276: F, t25284: F, t27228: F, t27230: F, t28337: F, t29623: F, t25220: F, t25232: F, t25243: F, t28330: F, t28333: F, t28335: F, t28336: F, t29616: F, t29618: F, t29620: F) -> (F,) {
    let t29627 = t7045 * t5993;
    let t29629 = t7025 * t5985;
    let t29631 = t7038 * t6019;
    let t29633 = t7045 * t6030;
    let t29635 = t25254 + t29623 / 16.0 - 0.50820002809285328226e-4 * t27228 + 0.40015750243531754508e-2 * t27230 + 0.85748036236139473945e-2 * t29627 - t29629 / 48.0 + t28337 + t25276 - t25284 - 0.42874018118069736972e-3 * t29631 - 0.17149607247227894789e-2 * t29633;
    let t29636 = t25220 - t25232 + t25243 + t28330 + 0.85748036236139473944e-3 * t29616 + 0.34299214494455789578e-2 * t29618 - 0.42874018118069736972e-3 * t29620 - t28335 + t28336 + t28333 + t29635;
    (t29636,)
}
