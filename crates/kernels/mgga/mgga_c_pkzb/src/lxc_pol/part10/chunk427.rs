//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 427/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk427<F: Float>(t114: F, t1613: F, t1504: F, t541: F, t1497: F, t1503: F, t1507: F) -> (F, F, F, F, F) {
    let t1614 = t114 * t1613;
    let t1615 = t1504 * t541;
    let t1618 = t1497 * t541;
    let t1621 = t114 * t1503;
    let t1622 = t1504 * t1507;
    (t1614, t1615, t1618, t1621, t1622)
}
