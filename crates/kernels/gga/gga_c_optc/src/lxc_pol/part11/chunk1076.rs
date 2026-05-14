//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1076/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1076<F: Float>(t1111: F, t17352: F, t9142: F, t17863: F, t2586: F, t1133: F, t15332: F, t4363: F, t1108: F, t17928: F, t4386: F, t9189: F, t17727: F, t45811: F, t1129: F, t17886: F) -> (F, F, F, F, F, F, F, F) {
    let t54389 = t1111 * t9142 * t17352;
    let t54391 = t2586 * t17863;
    let t54392 = t1133 * t54391;
    let t54394 = t4363 * t15332;
    let t54408 = t17928 * t1108;
    let t54430 = t4386 * t9189 * t17352;
    let t54451 = t45811 * t17727;
    let t54470 = t17886 * t1129;
    (t54389, t54391, t54392, t54394, t54408, t54430, t54451, t54470)
}
