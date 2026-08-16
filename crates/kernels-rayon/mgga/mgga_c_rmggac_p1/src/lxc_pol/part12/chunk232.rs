//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 232/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk232(t53: f64, t60: f64, t417: f64, t977: f64, t978: f64, t431: f64, t58: f64, t437: f64, t913: f64, t916: f64, t63: f64, t441: f64, t922: f64, t925: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t54 = t53 <= zeta_threshold;
    let t61 = t60 <= zeta_threshold;
    let t980 = t977 * t978 * t417;
    let t982 = 0.11696447245269292414e1_f64 * t431 * t980;
    let t983 = 1.0_f64 / t58;
    let t989 = piecewise3(t54, 0.0_f64, -2.0_f64 / 9.0_f64 * t983 * t913 + 2.0_f64 / 3.0_f64 * t437 * t916);
    let t990 = 1.0_f64 / t63;
    let t996 = piecewise3(t61, 0.0_f64, -2.0_f64 / 9.0_f64 * t990 * t922 + 2.0_f64 / 3.0_f64 * t441 * t925);
    (t980, t982, t983, t989, t990, t996)
}
