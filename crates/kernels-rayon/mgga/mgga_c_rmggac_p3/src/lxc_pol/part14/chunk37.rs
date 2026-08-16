//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 37/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk37(t88: f64, t90: f64, rho1: f64, tau1: f64) -> (f64, f64, f64) {
    let t91 = t90 * t88;
    let t94 = pow_1_3(rho1);
    let t95 = t94 * t94;
    let t97 = 1.0_f64 / t95 / rho1;
    let t98 = tau1 * t97;
    (t91, t95, t98)
}
