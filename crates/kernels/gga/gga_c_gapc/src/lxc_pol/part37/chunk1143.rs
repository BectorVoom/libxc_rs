//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1143/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1143<F: Float>(t30472: F, t3483: F, t3480: F, t9370: F, t15430: F, t3808: F, t10538: F, t28182: F, t12058: F, t4915: F, t687: F, t1112: F, t1616: F, t10794: F, t24007: F, t13296: F, t2470: F, t3832: F) -> (F, F, F, F, F, F, F, F) {
    let t36105 = 4.0 * t30472 * t3483;
    let t36109 = t3480 * t9370;
    let t36111 = 2.0 * t15430 * t3808;
    let t36113 = 6.0 * t28182 * t10538;
    let t36116 = 12.0 * t4915 * t12058 * t687;
    let t36119 = 2.0 * t1616 * t1112 * t9370;
    let t36127 = 6.0 * t24007 * t10794;
    let t36130 = 24.0 * t13296 * t3832 * t2470;
    (t36105, t36109, t36111, t36113, t36116, t36119, t36127, t36130)
}
