//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1161/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1161(t35915: f64, t1998: f64, t4503: f64, t5124: f64, t7647: f64, t31632: f64, t31634: f64, t31638: f64, t31640: f64, t31644: f64, t31658: f64, t31660: f64, t31663: f64, t35898: f64, t35904: f64, t35907: f64, t35910: f64, t35912: f64, t35914: f64) -> f64 {
    let t35916 = 0.305625e-1_f64 * t35915;
    let t35917 = t1998 * t4503;
    let t35918 = 0.17149607247227894789e-2_f64 * t35917;
    let t35919 = t7647 * t5124;
    let t35920 = 0.17149607247227894789e-2_f64 * t35919;
    let t35923 = t35898 - 0.80031500487063509016e-2_f64 * t31632 - 0.64311027177104605458e-2_f64 * t31634 + 0.47172138434406228102e-2_f64 * t31638 - 0.85748036236139473945e-2_f64 * t31640 - 0.22675591804667994222e-1_f64 * t31644 - t35904 - 0.10718504529517434243e-2_f64 * t35907 + t35910 + t35912 + t35914 + t35916 - t35918 + t35920 - 0.83861579438944405513e-2_f64 * t31658 + 0.94344276868812456204e-3_f64 * t31660 + t31663;
    t35923
}
