//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 135/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk135(t53: f64, t280: f64, t437: f64, t62: f64, zeta_threshold: f64) -> (f64, f64) {
    let t54 = t53 <= zeta_threshold;
    let t440 = piecewise3(t54, 0.0_f64, 2.0_f64 / 3.0_f64 * t437 * t280);
    let t441 = 1.0_f64 / t62;
    (t440, t441)
}
