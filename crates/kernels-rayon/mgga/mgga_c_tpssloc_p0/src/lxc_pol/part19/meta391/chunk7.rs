//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1478/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1478(t11153: f64, t1176: f64, t11881: f64, t45113: f64, t11773: f64, t1227: f64, t13969: f64, t11168: f64, t1174: f64, t3431: f64, t3540: f64, t3567: f64) -> (f64, f64, f64, f64, f64) {
    let t45192 = t1176 * t11153;
    let t45197 = t11881 * t45113;
    let t45211 = t1227 * t13969 * t11773;
    let t45222 = t1174 * t3431 * t11168;
    let t45224 = t3567 * t3540;
    (t45192, t45197, t45211, t45222, t45224)
}
