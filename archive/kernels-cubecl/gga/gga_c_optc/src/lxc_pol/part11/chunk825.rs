//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 825/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk825<F: Float>(t430: F, t5328: F, t4298: F, t4450: F, t4488: F, t2367: F, t5285: F, t1162: F, t5289: F, t1179: F, t15321: F, t5318: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15781 = t430 * t5328;
    let t15786 = t4298 * t5328;
    let t15826 = t4450 * t4488;
    let t15828 = t2367 * t5285;
    let t15829 = t1162 * t15828;
    let t15831 = t2367 * t5289;
    let t15832 = t1162 * t15831;
    let t15840 = t1179 * t15321;
    let t15843 = t2367 * t5318;
    (t15781, t15786, t15826, t15828, t15829, t15831, t15832, t15840, t15843)
}
