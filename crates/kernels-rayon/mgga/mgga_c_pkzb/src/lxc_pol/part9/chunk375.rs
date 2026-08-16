//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 375/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk375(t1485: f64, t49: f64, t55: f64, t63: f64, t1479: f64, t1482: f64, t482: f64, t1478: f64, t50: f64, t65: f64, t1480: f64, t1483: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1486 = t49 * t1485;
    let t1488 = 1.0_f64/f64::sqrt(t55);
    let t1489 = t1488 * t63;
    let t1490 = t1489 * t1479;
    let t1492 = t482 * t1482;
    let t1495 = t65 * t50 * t1478;
    let t1497 = -0.57538888888888888889e0_f64 * t1480 + 0.11507777777777777778e1_f64 * t1483 + 0.40256666666666666667e0_f64 * t1486 + 0.366775e-1_f64 * t1490 + 0.73355e-1_f64 * t1492 + 0.137975e0_f64 * t1495;
    (t1486, t1489, t1490, t1492, t1495, t1497)
}
