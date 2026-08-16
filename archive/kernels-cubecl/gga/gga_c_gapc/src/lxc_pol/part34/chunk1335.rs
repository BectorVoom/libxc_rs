//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1335/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1335<F: Float>(t3480: F, t9370: F, t15430: F, t3808: F, t10538: F, t28182: F, t12058: F, t4915: F, t687: F, t1112: F, t1616: F, t2011: F, t3822: F) -> (F, F, F, F, F, F) {
    let t36109 = t3480 * t9370;
    let t36111 = F::cast_from(2.0_f64) * t15430 * t3808;
    let t36113 = F::cast_from(6.0_f64) * t28182 * t10538;
    let t36116 = F::cast_from(12.0_f64) * t4915 * t12058 * t687;
    let t36119 = F::cast_from(2.0_f64) * t1616 * t1112 * t9370;
    let t36122 = F::cast_from(2.0_f64) * t1616 * t3822 * t2011;
    (t36109, t36111, t36113, t36116, t36119, t36122)
}
