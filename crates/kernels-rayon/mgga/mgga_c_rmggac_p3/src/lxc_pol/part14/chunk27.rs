//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 27/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk27(t60: f64, t62: f64, t56: f64, t59: f64, t49: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t61 = t60 <= zeta_threshold;
    let t63 = t62 * t60;
    let t64 = piecewise3(t61, t56, t63);
    let t65 = t59 + t64 - 2.0_f64;
    let t68 = 1.0_f64 / t49 / 2.0_f64;
    (t63, t65, t68)
}
