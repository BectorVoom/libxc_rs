//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1207/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1207(t43: f64, t50: f64, t48542: f64, t48544: f64, t48546: f64, t48548: f64, t48550: f64, t48552: f64, t48554: f64, t48556: f64, t48558: f64, t48560: f64, zeta_threshold: f64) -> f64 {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t48976 = piecewise3(t44, 0.0_f64, -56.0_f64 / 81.0_f64 * t48542 + 16.0_f64 / 9.0_f64 * t48544 - 2.0_f64 / 3.0_f64 * t48546 - 8.0_f64 / 9.0_f64 * t48548 + 2.0_f64 / 3.0_f64 * t48550);
    let t48983 = piecewise3(t51, 0.0_f64, -56.0_f64 / 81.0_f64 * t48552 + 16.0_f64 / 9.0_f64 * t48554 - 2.0_f64 / 3.0_f64 * t48556 - 8.0_f64 / 9.0_f64 * t48558 + 2.0_f64 / 3.0_f64 * t48560);
    let t48985 = t48976 / 2.0_f64 + t48983 / 2.0_f64;
    t48985
}
