//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 691/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk691<F: Float>(t1026: F, t1804: F, t1808: F, t1850: F, t3039: F, t122: F, t1266: F, t1034: F, t1040: F, t3061: F, t3065: F, t3060: F, t3072: F, t3076: F, t3138: F, t3144: F) -> (F, F, F, F, F, F, F) {
    let t8858 = t1804 * t1026;
    let t8859 = t8858 * t1808;
    let t8861 = t3039 * t1850;
    let t8863 = t1266 * t122;
    let t8864 = t8863 * t1034;
    let t8865 = t8864 * t1040;
    let t8867 = t3061 * t3065;
    let t8869 = t3060 * t3072;
    let t8870 = t8869 * t3076;
    let t8872 = t3060 * t3138;
    let t8873 = t8872 * t3144;
    (t8859, t8861, t8863, t8865, t8867, t8870, t8873)
}
