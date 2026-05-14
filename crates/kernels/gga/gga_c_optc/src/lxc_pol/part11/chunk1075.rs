//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1075/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1075<F: Float>(t1128: F, t17704: F, t8913: F, t1111: F, t17903: F, t24: F, t17907: F, t17723: F, t2586: F, t1133: F, t4356: F, t5110: F, t15327: F, t4380: F, t17922: F, t27031: F) -> (F, F, F, F, F, F, F, F) {
    let t54268 = t8913 * t1128 * t17704;
    let t54295 = t1111 * t24 * t17903;
    let t54298 = t1111 * t24 * t17907;
    let t54304 = t2586 * t17723;
    let t54305 = t1133 * t54304;
    let t54308 = t4356 * t5110;
    let t54317 = t15327 * t4380;
    let t54341 = t27031 * t17922;
    (t54268, t54295, t54298, t54304, t54305, t54308, t54317, t54341)
}
