//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1051/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1051<F: Float>(t30543: F, t9670: F, t1165: F, t39794: F, t604: F, t7413: F, t1181: F, t30856: F, t40215: F, t599: F, t35988: F, t35992: F, t35995: F, t35998: F, t36005: F, t36007: F, t36011: F, t36018: F, t36022: F, t36031: F, t36040: F, t37818: F, t37834: F, t37835: F) -> (F,) {
    let t40398 = t30543 * t9670;
    let t40403 = t7413 * t1165 * t604 * t39794;
    let t40408 = t30856 * t1181 * t599 * t40215;
    let t40410 = -0.12862205435420921092e-1 * t40398 + t37818 + t35988 + t35992 - 0.31448092289604152067e-2 * t35995 - t35998 - t36005 - t36007 - t36011 - 0.94344276868812456204e-3 * t40403 - t36018 + 0.62896184579208304136e-3 * t36022 - 0.64311027177104605458e-3 * t40408 - t36031 + t37834 + t37835 - t36040;
    (t40410,)
}
