//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1172/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1172(t142: f64, t4578: f64, t8806: f64, t4483: f64, t1318: f64, t361: f64, t7436: f64, t4582: f64, t31689: f64, t36005: f64, t36007: f64, t36011: f64, t36014: f64, t36018: f64, t36022: f64, t36026: f64, t36031: f64, t36032: f64, t36036: f64, t36040: f64, t36042: f64, t36044: f64) -> f64 {
    let t36047 = t8806 * t142 * t4578;
    let t36050 = t8806 * t142 * t4483;
    let t36053 = t7436 * t361 * t1318;
    let t36056 = t7436 * t142 * t4582;
    let t36058 = -t36005 - t36007 - t36011 + 0.62896184579208304136e-3_f64 * t36014 + 0.10718504529517434243e-2_f64 * t31689 - t36018 + 0.31448092289604152068e-3_f64 * t36022 - 0.15724046144802076034e-2_f64 * t36026 - t36031 + 0.6621875e-1_f64 * t36032 + 0.7640625e-2_f64 * t36036 - t36040 - t36042 + t36044 / 8.0_f64 + t36047 / 8.0_f64 + t36050 / 16.0_f64 + t36053 / 24.0_f64 + t36056 / 24.0_f64;
    t36058
}
