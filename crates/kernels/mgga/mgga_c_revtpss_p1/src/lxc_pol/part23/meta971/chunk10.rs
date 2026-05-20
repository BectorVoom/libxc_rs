//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3289/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3289<F: Float>(t14239: F, t22332: F, t10023: F, t22863: F, t686: F, t72: F, t1398: F, t14193: F, t1437: F, t22321: F, t47961: F, t47964: F, t5659: F, t5735: F, t74893: F, t74901: F, t74908: F, t820: F, t85638: F, t86054: F, t86374: F) -> F {
    let t86377 = t14239 * t22332;
    let t86381 = t10023 * t22863 * t72 * t686;
    let t86387 = -F::cast_from(0.29272321618148349057e-1_f64) * t74893 - F::cast_from(0.33133632253434461091e-3_f64) * t47961 - F::cast_from(0.19756347548806534796e1_f64) * t820 * t22321 * t5659 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t1437 * t86054 + F::cast_from(0.34697458558045176418e-2_f64) * t74901 + F::cast_from(0.9757440539382783019e-2_f64) * t86374 - F::cast_from(0.58544643236296698113e-1_f64) * t74908 - F::cast_from(0.29272321618148349057e-1_f64) * t86377 + F::cast_from(0.58544643236296698112e-1_f64) * t86381 - F::cast_from(0.11853808529283920877e2_f64) * t14193 * t5735 * t85638 * t1398 + t47964;
    t86387
}
