//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1186/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1186<F: Float>(t11975: F, t17688: F, t3116: F, t1111: F, t17666: F, t24: F, t17927: F, t861: F, t11937: F, t15622: F, t1128: F, t17704: F, t8913: F) -> (F, F, F, F, F) {
    let t54245 = t3116 * t11975 * t17688;
    let t54248 = t1111 * t24 * t17666;
    let t54252 = t17927 * t861;
    let t54261 = t11937 * t15622;
    let t54268 = t8913 * t1128 * t17704;
    (t54245, t54248, t54252, t54261, t54268)
}
