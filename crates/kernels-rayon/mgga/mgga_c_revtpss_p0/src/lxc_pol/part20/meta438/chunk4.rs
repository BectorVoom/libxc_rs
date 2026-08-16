//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1655/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1655(t1175: f64, t12552: f64, t43752: f64, t439: f64, t1160: f64, t12408: f64, t3519: f64, t3522: f64, t3444: f64, t3451: f64, t1156: f64, t12428: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t45174 = t1175 * t12552;
    let t45177 = t439 * t43752;
    let t45181 = t12408 * t1160;
    let t45186 = t3519 * t3519;
    let t45187 = 1.0_f64 / t45186;
    let t45188 = t439 * t45187;
    let t45189 = t3522 * t3522;
    let t45190 = 1.0_f64 / t45189;
    let t45194 = t3444 * t3451;
    let t45197 = t1156 * t12428;
    (t45174, t45177, t45181, t45187, t45188, t45190, t45194, t45197)
}
