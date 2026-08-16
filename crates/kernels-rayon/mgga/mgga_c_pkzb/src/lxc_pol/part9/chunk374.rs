//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 374/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk374(t1476: f64, t1478: f64, t1475: f64, t148: f64, t475: f64, t474: f64, t51: f64) -> (f64, f64, f64, f64, f64) {
    let t1479 = t1476 * t1478;
    let t1480 = t1475 * t1479;
    let t1482 = t475 * t148;
    let t1483 = t474 * t1482;
    let t1485 = t51 * t148;
    (t1479, t1480, t1482, t1483, t1485)
}
