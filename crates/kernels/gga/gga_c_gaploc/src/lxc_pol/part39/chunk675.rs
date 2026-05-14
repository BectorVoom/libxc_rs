//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 675/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk675<F: Float>(t12383: F, t12386: F, t12397: F, t12400: F, t12412: F, t10318: F, t2321: F, t9074: F, t3158: F, t993: F, t2268: F, t10268: F, t4261: F, t2854: F, t3085: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12787 = 9.0 / 256.0 * t12383;
    let t12788 = 9.0 / 8192.0 * t12386;
    let t12789 = 3.0 / 8192.0 * t12397;
    let t12790 = 3.0 / 256.0 * t12400;
    let t12791 = 2.0 * t12412;
    let t12797 = t10318 * t2321;
    let t12798 = t9074 * t12797;
    let t12799 = 0.23712505529730124666e-2 * t12798;
    let t12800 = t3158 * t993;
    let t12802 = 0.19918504644973304719e0 * t2268 * t12800;
    let t12803 = t4261 * t10268;
    let t12804 = t9074 * t12803;
    let t12805 = 0.47425011059460249332e-2 * t12804;
    let t12806 = t2854 * t3085;
    (t12787, t12788, t12789, t12790, t12791, t12797, t12799, t12800, t12802, t12803, t12805, t12806)
}
