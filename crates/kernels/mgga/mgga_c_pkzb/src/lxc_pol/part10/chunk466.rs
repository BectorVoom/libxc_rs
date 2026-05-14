//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 466/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk466<F: Float>(t135: F, t144: F, t1596: F, t1604: F, t1627: F, t1630: F, t1632: F, t1633: F, t1634: F, t1641: F, t1663: F, t1665: F, t1669: F, t1672: F, t1673: F, t1676: F, t1692: F, t1816: F, t560: F, t639: F) -> (F,) {
    let t1820 = -t135 * t144 * t1673 * t1676 + t135 * t144 * t1816 * t639 + 6.0 * t135 * t1633 * t1634 + 3.0 * t135 * t1692 * t560 - t1596 + t1604 + t1627 + t1630 - t1632 + t1641 + t1663 + t1665 + t1669 - t1672;
    (t1820,)
}
