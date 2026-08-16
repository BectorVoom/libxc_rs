//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 846/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk846(t41789: f64, t1550: f64, t41548: f64, t1978: f64, t7228: f64, t8511: f64, t1982: f64, t7428: f64, t16156: f64, t9198: f64, t388: f64, t575: f64, t7933: f64, t7934: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t41790 = 0.15965655602485078085e0_f64 * t41789;
    let t41791 = t1550 * t41548;
    let t41792 = 0.15965655602485078085e0_f64 * t41791;
    let t41799 = t8511 * t7228 * t1978;
    let t41811 = t8511 * t7428 * t1982;
    let t41812 = 0.19863479950205658386e-4_f64 * t41811;
    let t41813 = t16156 * t9198;
    let t41817 = t7933 * t7934 * t388 * t575;
    (t41790, t41792, t41799, t41812, t41813, t41817)
}
