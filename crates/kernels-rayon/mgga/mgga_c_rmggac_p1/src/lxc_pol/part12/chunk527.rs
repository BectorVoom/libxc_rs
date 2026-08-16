//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 527/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk527(t2074: f64, t352: f64, t262: f64, t7192: f64, t22: f64, t880: f64, t507: f64) -> (f64, f64, f64, f64, f64) {
    let t7193 = t2074 * t352;
    let t7194 = t262 * t7193;
    let t7195 = t7192 * t7194;
    let t7196 = 0.27274661654245341728e-1_f64 * t7195;
    let t7197 = t880 * t22;
    let t7198 = t507 * t7197;
    (t7193, t7194, t7196, t7197, t7198)
}
