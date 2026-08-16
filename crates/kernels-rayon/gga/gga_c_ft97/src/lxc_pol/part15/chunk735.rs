//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 735/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk735(t20352: f64, t582: f64, t20039: f64, t3506: f64, t20655: f64, t24: f64, t586: f64, t20660: f64, t9236: f64, t1985: f64, t3518: f64, t4714: f64) -> (f64, f64, f64, f64, f64) {
    let t20810 = t582 * t20352;
    let t20813 = t3506 * t20039;
    let t20818 = t24 * t586 * t20655;
    let t20823 = t24 * t9236 * t20660;
    let t20827 = t1985 * t3518 * t4714;
    (t20810, t20813, t20818, t20823, t20827)
}
