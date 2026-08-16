//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 114/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk114(t338: f64, t352: f64, t118: f64, t323: f64, t335: f64) -> (f64, f64, f64) {
    let t353 = t338 * t352;
    let t354 = t118 * t353;
    let t356 = -0.59871208509319042821e-1_f64 * t323 + 0.59871208509319042821e-1_f64 * t335 + 0.19957069503106347607e-1_f64 * t354;
    (t353, t354, t356)
}
