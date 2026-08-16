//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 854/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk854(t1608: f64, t7905: f64, t8007: f64, t1620: f64, t6: f64, t7900: f64, t8010: f64, t8018: f64, t7837: f64, t8008: f64, t7839: f64, t7929: f64) -> (f64, f64, f64, f64, f64) {
    let t37504 = t1608 * t8007 * t7905;
    let t37506 = t7900 * t6 * t1620;
    let t37509 = t8010 * t8018;
    let t37518 = t7837 * t8008;
    let t37519 = t7929 * t7839;
    (t37504, t37506, t37509, t37518, t37519)
}
