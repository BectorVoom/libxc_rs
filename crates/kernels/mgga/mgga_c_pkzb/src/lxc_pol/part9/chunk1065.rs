//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1065/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1065<F: Float>(t1527: F, t1598: F, t1601: F, t479: F, t490: F, t1545: F, t1662: F, t1542: F, t1628: F, t1581: F, t1571: F, t114: F, t1497: F, t1570: F, t1590: F, t1615: F, t16193: F, t1621: F, t16273: F, t16283: F, t16287: F, t16290: F, t16486: F, t16489: F, t16513: F, t16517: F, t16526: F, t16540: F, t16557: F, t16560: F, t16603: F, t4915: F, t4966: F, t4979: F, t525: F, t526: F) -> (F, F, F, F, F, F) {
    let t16631 = F::new(0.34367190188705947437e1) * t479 * t1598 * t1527 * t1601 * t490;
    let t16632 = t1545 * t1662;
    let t16638 = t1542 * t1628;
    let t16654 = t1581 * t1581;
    let t16662 = t1571 * t1571;
    let t16666 = t16193 + t16273 - F::new(8.0) * t1570 * t4979 * t525 - F::new(0.11579025239058625248e4) * t4966 * t1590 * t1581 - F::new(0.12304822629859687989e5) * t114 * t16603 * t16540 * t4915 - t16283 - t16287 + t16290 + t16486 + t16489 + t16513 + F::new(0.21053605041484726346e2) * t1621 * t1615 * t1497 - t16517 - t16526 - F::new(6.0) * t1570 * t16654 * t526 + F::new(0.91082604192152556044e5) * t114 * t16557 * t16540 * t16560 - F::new(24.0) * t4966 * t16662 * t526;
    (t16631, t16632, t16638, t16654, t16662, t16666)
}
