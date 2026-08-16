//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1906/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1906<F: Float>(t22761: F, t6390: F, t2002: F, t6378: F, t559: F, t6422: F, t6945: F, t6427: F, t6952: F, t6431: F, t1831: F, t26257: F) -> (F, F, F, F, F, F, F) {
    let t28085 = t22761 * t6390;
    let t28088 = t6378 * t2002;
    let t28089 = t28088 * t559;
    let t28091 = t6945 * t6422;
    let t28093 = t6952 * t6427;
    let t28095 = t6952 * t6431;
    let t28097 = t26257 * t1831;
    (t28085, t28088, t28089, t28091, t28093, t28095, t28097)
}
