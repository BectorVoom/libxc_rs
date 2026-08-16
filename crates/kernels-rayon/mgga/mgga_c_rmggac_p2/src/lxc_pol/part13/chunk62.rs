//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 62/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk62(t53: f64, t60: f64, t155: f64, t184: f64, t156: f64, t181: f64, t55: f64, t57: f64, t62: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t54 = t53 <= zeta_threshold;
    let t61 = t60 <= zeta_threshold;
    let t185 = t155 * t184;
    let t187 = 0.19751673498613801407e-1_f64 * t156 * t181;
    let t188 = t55 * t55;
    let t189 = t57 * t57;
    let t190 = piecewise3(t54, t188, t189);
    let t191 = t62 * t62;
    let t192 = piecewise3(t61, t188, t191);
    let t194 = t190 / 2.0_f64 + t192 / 2.0_f64;
    (t185, t187, t189, t191, t194)
}
