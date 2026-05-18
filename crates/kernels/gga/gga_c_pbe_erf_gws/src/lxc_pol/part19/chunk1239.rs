//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1239/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1239<F: Float>(t54101: F, t854: F, t3228: F, t51465: F, t3224: F, t1114: F, t51266: F, t1150: F, t51200: F, t14028: F, t3295: F, t14024: F, t3113: F) -> (F, F, F, F, F, F, F) {
    let t54102 = t854 * t54101;
    let t54113 = t51465 * t3228;
    let t54117 = t51465 * t3224;
    let t54119 = t1114 * t51266;
    let t54126 = t51200 * t1150;
    let t54128 = t14028 * t3295;
    let t54135 = t3113 * t14024;
    (t54102, t54113, t54117, t54119, t54126, t54128, t54135)
}
