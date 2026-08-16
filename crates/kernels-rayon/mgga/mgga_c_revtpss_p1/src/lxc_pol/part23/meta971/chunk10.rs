//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3289/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3289(t14239: f64, t22332: f64, t10023: f64, t22863: f64, t686: f64, t72: f64, t1398: f64, t14193: f64, t1437: f64, t22321: f64, t47961: f64, t47964: f64, t5659: f64, t5735: f64, t74893: f64, t74901: f64, t74908: f64, t820: f64, t85638: f64, t86054: f64, t86374: f64) -> f64 {
    let t86377 = t14239 * t22332;
    let t86381 = t10023 * t22863 * t72 * t686;
    let t86387 = -0.29272321618148349057e-1_f64 * t74893 - 0.33133632253434461091e-3_f64 * t47961 - 0.19756347548806534796e1_f64 * t820 * t22321 * t5659 - 0.65854491829355115987e0_f64 * t820 * t1437 * t86054 + 0.34697458558045176418e-2_f64 * t74901 + 0.9757440539382783019e-2_f64 * t86374 - 0.58544643236296698113e-1_f64 * t74908 - 0.29272321618148349057e-1_f64 * t86377 + 0.58544643236296698112e-1_f64 * t86381 - 0.11853808529283920877e2_f64 * t14193 * t5735 * t85638 * t1398 + t47964;
    t86387
}
