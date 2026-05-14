//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1224/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1224<F: Float>(t1128: F, t8921: F, t8923: F, t1137: F, t3843: F, t1133: F, t3152: F, t7878: F, t8960: F, t8962: F, t1122: F, t1135: F, t1: F, t15654: F, t9044: F, t123: F, t17919: F, t1900: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26999 = t8921 * t1128 * t8923;
    let t27001 = t3843 * t1137;
    let t27002 = t1133 * t27001;
    let t27004 = t7878 * t3152;
    let t27005 = t1133 * t27004;
    let t27008 = t8960 * t1128 * t8962;
    let t27010 = t1135 * t1122;
    let t27011 = t27010 * t1;
    let t27012 = t15654 * t9044;
    let t27017 = t17919 * t1900 * t123;
    (t26999, t27001, t27002, t27004, t27005, t27008, t27011, t27012, t27017)
}
