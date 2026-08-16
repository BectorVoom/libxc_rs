//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1005/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1005(t2198: f64, t8232: f64, t1882: f64, t9333: f64, t9324: f64, t9337: f64, t9329: f64, t9295: f64, t9434: f64, t143: f64, t38052: f64, t9408: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t40979 = t8232 * t2198;
    let t40981 = t1882 * t9333;
    let t40983 = t1882 * t9324;
    let t40985 = t1882 * t9337;
    let t40987 = t1882 * t9329;
    let t40989 = t1882 * t9295;
    let t40991 = t1882 * t9434;
    let t41002 = t38052 * t143;
    let t41019 = t1882 * t9408;
    (t40979, t40981, t40983, t40985, t40987, t40989, t40991, t41002, t41019)
}
