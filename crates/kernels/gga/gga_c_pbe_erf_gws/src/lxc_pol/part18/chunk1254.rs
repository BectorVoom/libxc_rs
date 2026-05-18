//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1254/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1254<F: Float>(t54117: F, t1114: F, t51266: F, t1150: F, t51200: F, t14028: F, t3295: F, t14024: F, t3113: F, t3123: F, t51430: F, t14538: F, t51329: F) -> (F, F, F, F, F, F, F) {
    let t54118 = F::new(7.0) / F::new(288.0) * t54117;
    let t54119 = t1114 * t51266;
    let t54126 = t51200 * t1150;
    let t54128 = t14028 * t3295;
    let t54129 = F::new(7.0) / F::new(576.0) * t54128;
    let t54135 = t3113 * t14024;
    let t54136 = F::new(7.0) / F::new(144.0) * t54135;
    let t54152 = t3123 * t51430;
    let t54153 = F::new(7.0) / F::new(144.0) * t54152;
    let t54166 = t14538 * t51329;
    (t54118, t54119, t54126, t54129, t54136, t54153, t54166)
}
