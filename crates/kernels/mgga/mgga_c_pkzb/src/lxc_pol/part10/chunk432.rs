//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 432/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk432<F: Float>(t1628: F, t83: F, t501: F, t513: F, t142: F, t192: F, t568: F) -> (F, F, F, F, F, F) {
    let t1629 = t83 * t1628;
    let t1630 = 2.0 * t1629;
    let t1631 = t501 * t513;
    let t1632 = 8.0 * t1631;
    let t1633 = t142 * t192;
    let t1634 = t568 * t568;
    (t1629, t1630, t1631, t1632, t1633, t1634)
}
