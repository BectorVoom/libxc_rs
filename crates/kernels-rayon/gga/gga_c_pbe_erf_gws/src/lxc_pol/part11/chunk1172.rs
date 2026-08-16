//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1172/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1172(t3354: f64, t9793: f64, t1528: f64, t47733: f64, t12355: f64, t2485: f64, t47372: f64, t478: f64, t48542: f64, t48544: f64, t48546: f64, t48548: f64, t48550: f64, t48552: f64) -> (f64, f64, f64, f64, f64) {
    let t48554 = t9793 * t3354;
    let t48556 = t1528 * t47733;
    let t48558 = t2485 * t12355;
    let t48560 = t478 * t47372;
    let t48562 = -28.0_f64 / 81.0_f64 * t48542 + 8.0_f64 / 9.0_f64 * t48544 - t48546 / 3.0_f64 - 4.0_f64 / 9.0_f64 * t48548 + t48550 / 3.0_f64 - 28.0_f64 / 81.0_f64 * t48552 + 8.0_f64 / 9.0_f64 * t48554 - t48556 / 3.0_f64 - 4.0_f64 / 9.0_f64 * t48558 + t48560 / 3.0_f64;
    (t48554, t48556, t48558, t48560, t48562)
}
