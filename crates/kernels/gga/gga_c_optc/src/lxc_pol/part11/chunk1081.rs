//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1081/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1081<F: Float>(t16011: F, t4512: F, t18019: F, t3244: F, t9142: F, t11885: F, t18012: F, t1179: F, t18075: F, t2586: F, t12802: F, t16004: F, t12581: F, t15983: F, t15986: F, t4492: F) -> (F, F, F, F, F, F, F) {
    let t54853 = t16011 * t4512;
    let t54904 = t3244 * t9142 * t18019;
    let t54911 = t3244 * t11885 * t18012;
    let t54926 = t1179 * t2586 * t18075;
    let t54941 = t12802 * t16004;
    let t54944 = t12581 * t15983;
    let t54947 = t4492 * t15986;
    (t54853, t54904, t54911, t54926, t54941, t54944, t54947)
}
