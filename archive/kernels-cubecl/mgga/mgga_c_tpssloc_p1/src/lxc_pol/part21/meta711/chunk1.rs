//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2547/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2547<F: Float>(t1036: F, t13942: F, t3047: F, t4616: F, t10890: F, t14507: F, t1041: F, t13969: F, t14188: F, t1020: F, t14489: F, t248: F, t3101: F) -> (F, F, F, F, F) {
    let t49734 = t13942 * t1036;
    let t49740 = t4616 * t3047;
    let t49743 = t14507 * t10890;
    let t49748 = t1041 * t13969 * t14188;
    let t49757 = t1020 * t248 * t3101 * t14489;
    (t49734, t49740, t49743, t49748, t49757)
}
