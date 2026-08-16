//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 177/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk177(t53: f64, t60: f64, t155: f64, t578: f64, t181: f64, t577: f64, t437: f64, t521: f64, t441: f64, t525: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t54 = t53 <= zeta_threshold;
    let t61 = t60 <= zeta_threshold;
    let t579 = t155 * t578;
    let t581 = 0.19751673498613801407e-1_f64 * t577 * t181;
    let t584 = piecewise3(t54, 0.0_f64, 2.0_f64 / 3.0_f64 * t437 * t521);
    let t587 = piecewise3(t61, 0.0_f64, 2.0_f64 / 3.0_f64 * t441 * t525);
    let t589 = t584 / 2.0_f64 + t587 / 2.0_f64;
    (t579, t581, t589)
}
