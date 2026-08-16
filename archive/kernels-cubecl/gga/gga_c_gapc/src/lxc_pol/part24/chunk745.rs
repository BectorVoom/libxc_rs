//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 745/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk745<F: Float>(t1808: F, t8858: F, t1850: F, t3039: F, t122: F, t1266: F, t1034: F, t1040: F, t3061: F, t3065: F, t3060: F, t3072: F) -> (F, F, F, F, F, F) {
    let t8859 = t8858 * t1808;
    let t8861 = t3039 * t1850;
    let t8863 = t1266 * t122;
    let t8864 = t8863 * t1034;
    let t8865 = t8864 * t1040;
    let t8867 = t3061 * t3065;
    let t8869 = t3060 * t3072;
    (t8859, t8861, t8863, t8865, t8867, t8869)
}
