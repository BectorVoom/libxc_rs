//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2680/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2680<F: Float>(t2782: F, t4086: F, t49280: F, t543: F, t10069: F, t14225: F, t1399: F, t4004: F, t47348: F, t47351: F, t47352: F, t47354: F, t47359: F, t49205: F, t49268: F, t49274: F, t49276: F, t5675: F, t5745: F, t820: F) -> F {
    let t49283 = t2782 * t4086 * t49280 * t543;
    let t49289 = t10069 * t14225;
    let t49290 = F::cast_from(0.21951497276451705329e-1_f64) * t49289;
    let t49293 = -F::cast_from(0.19756347548806534796e1_f64) * t820 * t49268 * t1399 - t49274 + F::cast_from(0.58911598146606471822e-3_f64) * t47348 + F::cast_from(0.39512695097613069591e1_f64) * t820 * t49276 * t4004 + F::cast_from(0.16463622957338778996e-1_f64) * t49283 - t47351 + F::cast_from(0.7805952431506226415e-2_f64) * t47352 + F::cast_from(0.79025390195226139182e1_f64) * t5745 * t49205 * t5675 - t49290 - F::cast_from(0.29272321618148349057e-1_f64) * t47354 - F::cast_from(0.9757440539382783019e-2_f64) * t47359;
    t49293
}
