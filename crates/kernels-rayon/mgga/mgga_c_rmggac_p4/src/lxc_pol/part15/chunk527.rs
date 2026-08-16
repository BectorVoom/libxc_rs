//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 527/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk527(t638: f64, t641: f64, t7184: f64, t338: f64, t837: f64, t22: f64, t235: f64) -> (f64, f64, f64, f64) {
    let t7186 = t638 * t7184 * t641;
    let t7190 = t837 * t338;
    let t7191 = t7190 * t22;
    let t7192 = t235 * t7191;
    (t7186, t7190, t7191, t7192)
}
