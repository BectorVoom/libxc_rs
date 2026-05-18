//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1007/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1007<F: Float>(t3080: F, t862: F, t1189: F, t2278: F, t3103: F, t870: F, t1197: F, t2273: F, t2258: F, t3106: F, t2281: F, t3102: F) -> (F, F, F, F, F, F) {
    let t8115 = t3080 * t862;
    let t8120 = t1189 * t2278;
    let t8129 = t3103 * t870;
    let t8132 = t1197 * t2273;
    let t8135 = t3106 * t2258;
    let t8138 = t3102 * t2281;
    (t8115, t8120, t8129, t8132, t8135, t8138)
}
