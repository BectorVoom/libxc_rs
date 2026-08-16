//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 730/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk730(t128: f64, t2155: f64, t131: f64, t133: f64, t155: f64, t6880: f64, t696: f64, t6884: f64, t108: f64, t2156: f64, t110: f64, t146: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6990 = 1.0_f64 / t2155 / t128;
    let t6991 = t6990 * t131;
    let t6993 = t155 * t6991 * t133;
    let t6994 = t696 * t6880;
    let t6997 = t696 * t6884;
    let t7000 = t2156 * t108;
    let t7002 = t146 * t7000 * t110;
    (t6990, t6991, t6993, t6994, t6997, t7000, t7002)
}
