//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2680/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2680(t2782: f64, t4086: f64, t49280: f64, t543: f64, t10069: f64, t14225: f64, t1399: f64, t4004: f64, t47348: f64, t47351: f64, t47352: f64, t47354: f64, t47359: f64, t49205: f64, t49268: f64, t49274: f64, t49276: f64, t5675: f64, t5745: f64, t820: f64) -> f64 {
    let t49283 = t2782 * t4086 * t49280 * t543;
    let t49289 = t10069 * t14225;
    let t49290 = 0.21951497276451705329e-1_f64 * t49289;
    let t49293 = -0.19756347548806534796e1_f64 * t820 * t49268 * t1399 - t49274 + 0.58911598146606471822e-3_f64 * t47348 + 0.39512695097613069591e1_f64 * t820 * t49276 * t4004 + 0.16463622957338778996e-1_f64 * t49283 - t47351 + 0.7805952431506226415e-2_f64 * t47352 + 0.79025390195226139182e1_f64 * t5745 * t49205 * t5675 - t49290 - 0.29272321618148349057e-1_f64 * t47354 - 0.9757440539382783019e-2_f64 * t47359;
    t49293
}
