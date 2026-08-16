//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1190/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1190(t30543: f64, t9670: f64, t1165: f64, t39794: f64, t604: f64, t7413: f64, t1181: f64, t30856: f64, t40215: f64, t599: f64, t35988: f64, t35992: f64, t35995: f64, t35998: f64, t36005: f64, t36007: f64, t36011: f64, t36018: f64, t36022: f64, t36031: f64, t36040: f64, t37818: f64, t37834: f64, t37835: f64) -> f64 {
    let t40398 = t30543 * t9670;
    let t40403 = t7413 * t1165 * t604 * t39794;
    let t40408 = t30856 * t1181 * t599 * t40215;
    let t40410 = -0.12862205435420921092e-1_f64 * t40398 + t37818 + t35988 + t35992 - 0.31448092289604152067e-2_f64 * t35995 - t35998 - t36005 - t36007 - t36011 - 0.94344276868812456204e-3_f64 * t40403 - t36018 + 0.62896184579208304136e-3_f64 * t36022 - 0.64311027177104605458e-3_f64 * t40408 - t36031 + t37834 + t37835 - t36040;
    t40410
}
