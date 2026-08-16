//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1189/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1189<F: Float>(t17727: F, t45811: F, t1129: F, t17886: F, t15696: F, t4310: F, t12068: F, t17344: F, t4386: F, t15227: F, t15693: F, t15321: F, t4369: F) -> (F, F, F, F, F, F, F) {
    let t54451 = t45811 * t17727;
    let t54470 = t17886 * t1129;
    let t54472 = t4310 * t15696;
    let t54477 = t4386 * t12068 * t17344;
    let t54509 = t4310 * t15227;
    let t54511 = t4310 * t15693;
    let t54518 = t4369 * t15321;
    (t54451, t54470, t54472, t54477, t54509, t54511, t54518)
}
