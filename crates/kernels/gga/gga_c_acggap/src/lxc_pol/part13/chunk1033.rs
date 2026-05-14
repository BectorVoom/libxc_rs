//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1033/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1033<F: Float>(t36041: F, t1314: F, t361: F, t8806: F, t142: F, t4578: F, t4483: F, t1318: F, t7436: F, t4582: F, t31689: F, t36005: F, t36007: F, t36011: F, t36014: F, t36018: F, t36022: F, t36026: F, t36031: F, t36032: F, t36036: F, t36040: F) -> (F,) {
    let t36042 = 7.0 / 72.0 * t36041;
    let t36044 = t8806 * t361 * t1314;
    let t36047 = t8806 * t142 * t4578;
    let t36050 = t8806 * t142 * t4483;
    let t36053 = t7436 * t361 * t1318;
    let t36056 = t7436 * t142 * t4582;
    let t36058 = -t36005 - t36007 - t36011 + 0.62896184579208304136e-3 * t36014 + 0.10718504529517434243e-2 * t31689 - t36018 + 0.31448092289604152068e-3 * t36022 - 0.15724046144802076034e-2 * t36026 - t36031 + 0.6621875e-1 * t36032 + 0.7640625e-2 * t36036 - t36040 - t36042 + t36044 / 8.0 + t36047 / 8.0 + t36050 / 16.0 + t36053 / 24.0 + t36056 / 24.0;
    (t36058,)
}
