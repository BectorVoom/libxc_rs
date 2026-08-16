//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 968/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk968(t41227: f64, t8746: f64, t41055: f64, t851: f64, t41035: f64, t854: f64, t3826: f64, t39688: f64, t3810: f64, t39684: f64, t39879: f64, t40920: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41228 = t8746 * t41227;
    let t41230 = t851 * t41055;
    let t41233 = t854 * t41035;
    let t41235 = t3826 * t39688;
    let t41237 = t3810 * t39684;
    let t41239 = t3826 * t39879;
    let t41241 = t3810 * t40920;
    (t41228, t41230, t41233, t41235, t41237, t41239, t41241)
}
