//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 226/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk226(t53: f64, t60: f64, t57: f64, t912: f64, t913: f64, t916: f64, t191: f64, t284: f64, t62: f64, zeta_threshold: f64) -> (f64, f64, f64, f64) {
    let t54 = t53 <= zeta_threshold;
    let t61 = t60 <= zeta_threshold;
    let t920 = piecewise3(t54, 0.0_f64, 4.0_f64 / 9.0_f64 * t912 * t913 + 4.0_f64 / 3.0_f64 * t57 * t916);
    let t921 = 1.0_f64 / t191;
    let t922 = t284 * t284;
    let t925 = -t916;
    let t929 = piecewise3(t61, 0.0_f64, 4.0_f64 / 9.0_f64 * t921 * t922 + 4.0_f64 / 3.0_f64 * t62 * t925);
    let t930 = t920 + t929;
    (t921, t922, t925, t930)
}
