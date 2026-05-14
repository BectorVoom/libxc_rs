//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1104/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1104<F: Float>(t3179: F, t51291: F, t854: F, t3228: F, t51465: F, t3224: F, t1114: F, t51266: F, t1150: F, t51200: F, t14028: F, t3295: F, t14024: F, t3113: F, t3123: F, t51430: F) -> (F, F, F, F, F, F, F, F, F) {
    let t54101 = t51291 * t3179;
    let t54102 = t854 * t54101;
    let t54103 = 7.0 / 72.0 * t54102;
    let t54113 = t51465 * t3228;
    let t54114 = 7.0 / 288.0 * t54113;
    let t54117 = t51465 * t3224;
    let t54118 = 7.0 / 288.0 * t54117;
    let t54119 = t1114 * t51266;
    let t54126 = t51200 * t1150;
    let t54128 = t14028 * t3295;
    let t54129 = 7.0 / 576.0 * t54128;
    let t54135 = t3113 * t14024;
    let t54136 = 7.0 / 144.0 * t54135;
    let t54152 = t3123 * t51430;
    (t54101, t54103, t54114, t54118, t54119, t54126, t54129, t54136, t54152)
}
