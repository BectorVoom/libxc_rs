//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 77/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk77(t217: f64, t221: f64, t28: f64, t212: f64, t215: f64) -> (f64, f64, f64) {
    let t222 = t28 * t217 * t221;
    let t225 = 1.0_f64 + 0.27439556402611977244e-1_f64 * t212 * t215 * t222;
    let t226 = pow_1_4(t225);
    (t222, t225, t226)
}
