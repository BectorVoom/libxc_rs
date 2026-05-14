//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 971/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk971<F: Float>(t34296: F, t34298: F, t34305: F, t34308: F, t37012: F, t39141: F, t39143: F, t39145: F, t39147: F, t39151: F, t39155: F, t39160: F, t39162: F, t39167: F, t39169: F, t39171: F, t39173: F, t39176: F) -> (F,) {
    let t39178 = -0.94344276868812456204e-3 * t39141 + 0.68598428988911579156e-2 * t39143 + 0.34299214494455789578e-2 * t39145 - 0.17149607247227894789e-2 * t39147 - 0.62896184579208304136e-3 * t39151 - 0.7862023072401038017e-3 * t39155 + 0.31448092289604152068e-3 * t39160 - 0.64311027177104605458e-2 * t39162 - 0.47172138434406228102e-3 * t39167 - t39169 / 48.0 - t39171 / 96.0 - 0.40015750243531754507e-2 * t39173 + t37012 + t34296 - t34298 + 0.62896184579208304135e-3 * t34305 + t34308 - 0.10718504529517434243e-2 * t39176;
    (t39178,)
}
