//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 167/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk167(t521: f64, t90: f64, t95: f64, t101: f64, t102: f64, t320: f64, t87: f64, t98: f64, rho1: f64, tau1: f64) -> (f64, f64, f64, f64, f64) {
    let t537 = t521 / 2.0_f64;
    let t538 = t90 * t537;
    let t541 = rho1 * rho1;
    let t543 = 1.0_f64 / t95 / t541;
    let t544 = tau1 * t543;
    let t547 = -t537;
    let t548 = t101 * t547;
    let t551 = 10.0_f64 / 3.0_f64 * t87 * t538 - 10.0_f64 / 3.0_f64 * t544 * t102 + 10.0_f64 / 3.0_f64 * t98 * t548 + t320;
    (t537, t538, t544, t547, t551)
}
