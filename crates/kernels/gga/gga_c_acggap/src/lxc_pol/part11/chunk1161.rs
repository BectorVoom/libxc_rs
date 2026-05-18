//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1161/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1161<F: Float>(t35915: F, t1998: F, t4503: F, t5124: F, t7647: F, t31632: F, t31634: F, t31638: F, t31640: F, t31644: F, t31658: F, t31660: F, t31663: F, t35898: F, t35904: F, t35907: F, t35910: F, t35912: F, t35914: F) -> F {
    let t35916 = F::new(0.305625e-1) * t35915;
    let t35917 = t1998 * t4503;
    let t35918 = F::new(0.17149607247227894789e-2) * t35917;
    let t35919 = t7647 * t5124;
    let t35920 = F::new(0.17149607247227894789e-2) * t35919;
    let t35923 = t35898 - F::new(0.80031500487063509016e-2) * t31632 - F::new(0.64311027177104605458e-2) * t31634 + F::new(0.47172138434406228102e-2) * t31638 - F::new(0.85748036236139473945e-2) * t31640 - F::new(0.22675591804667994222e-1) * t31644 - t35904 - F::new(0.10718504529517434243e-2) * t35907 + t35910 + t35912 + t35914 + t35916 - t35918 + t35920 - F::new(0.83861579438944405513e-2) * t31658 + F::new(0.94344276868812456204e-3) * t31660 + t31663;
    t35923
}
