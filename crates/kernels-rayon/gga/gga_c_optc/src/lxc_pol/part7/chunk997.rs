//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 997/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk997(t188: f64, t21979: f64, t757: f64, t103: f64, t193: f64, t197: f64, t2078: f64, t102: f64, t652: f64, t751: f64, t133: f64, t1928: f64) -> (f64, f64, f64, f64) {
    let t21981 = t188 * t21979 * t757;
    let t21988 = 261800.0_f64 / 729.0_f64 * t193 * t2078 * t103 * t197;
    let t21989 = t652 * t102;
    let t21991 = t193 * t21989 * t751;
    let t21995 = t193 * t133 * t1928 * t197;
    (t21981, t21988, t21991, t21995)
}
