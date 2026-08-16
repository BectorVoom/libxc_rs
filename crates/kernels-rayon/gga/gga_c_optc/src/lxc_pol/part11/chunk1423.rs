//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1423/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1423(t59083: f64, t59155: f64, t59189: f64, t59432: f64, t1136: f64, t55927: f64, t894: f64, t27189: f64, t55917: f64, t1114: f64, t27083: f64, t27037: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t59434 = t59083 + t59155 + t59189 + t59432;
    let t59448 = t894 * t1136 * t55927;
    let t59452 = t894 * t27189 * t55917;
    let t59458 = t1114 * t55927;
    let t59462 = t27083 * t55917;
    let t59468 = t27037 * t55917;
    (t59434, t59448, t59452, t59458, t59462, t59468)
}
