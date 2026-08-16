//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 445/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk445(t135: f64, t144: f64, t1596: f64, t1604: f64, t1627: f64, t1630: f64, t1632: f64, t1633: f64, t1634: f64, t1641: f64, t1663: f64, t1665: f64, t1669: f64, t1672: f64, t1673: f64, t1676: f64, t1692: f64, t1816: f64, t560: f64, t639: f64) -> f64 {
    let t1820 = -t135 * t144 * t1673 * t1676 + t135 * t144 * t1816 * t639 + 6.0_f64 * t135 * t1633 * t1634 + 3.0_f64 * t135 * t1692 * t560 - t1596 + t1604 + t1627 + t1630 - t1632 + t1641 + t1663 + t1665 + t1669 - t1672;
    t1820
}
