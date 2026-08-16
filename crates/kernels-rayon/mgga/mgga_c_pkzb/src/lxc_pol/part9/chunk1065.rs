//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1065/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1065(t1527: f64, t1598: f64, t1601: f64, t479: f64, t490: f64, t1545: f64, t1662: f64, t1542: f64, t1628: f64, t1581: f64, t1571: f64, t114: f64, t1497: f64, t1570: f64, t1590: f64, t1615: f64, t16193: f64, t1621: f64, t16273: f64, t16283: f64, t16287: f64, t16290: f64, t16486: f64, t16489: f64, t16513: f64, t16517: f64, t16526: f64, t16540: f64, t16557: f64, t16560: f64, t16603: f64, t4915: f64, t4966: f64, t4979: f64, t525: f64, t526: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16631 = 0.34367190188705947437e1_f64 * t479 * t1598 * t1527 * t1601 * t490;
    let t16632 = t1545 * t1662;
    let t16638 = t1542 * t1628;
    let t16654 = t1581 * t1581;
    let t16662 = t1571 * t1571;
    let t16666 = t16193 + t16273 - 8.0_f64 * t1570 * t4979 * t525 - 0.11579025239058625248e4_f64 * t4966 * t1590 * t1581 - 0.12304822629859687989e5_f64 * t114 * t16603 * t16540 * t4915 - t16283 - t16287 + t16290 + t16486 + t16489 + t16513 + 0.21053605041484726346e2_f64 * t1621 * t1615 * t1497 - t16517 - t16526 - 6.0_f64 * t1570 * t16654 * t526 + 0.91082604192152556044e5_f64 * t114 * t16557 * t16540 * t16560 - 24.0_f64 * t4966 * t16662 * t526;
    (t16631, t16632, t16638, t16654, t16662, t16666)
}
