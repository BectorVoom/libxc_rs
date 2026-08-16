//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 551/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk551(t14444: f64, t352: f64, t8940: f64, t321: f64, t5148: f64, t36: f64, t698: f64) -> (f64, f64, f64) {
    let t14445 = t14444 * t352;
    let t14447 = 0.11974241701863808564e0_f64 * t8940 * t14445;
    let t14448 = t14444 * t321;
    let t14450 = 0.11974241701863808564e0_f64 * t5148 * t14448;
    let t14451 = t698 * t36;
    (t14447, t14450, t14451)
}
