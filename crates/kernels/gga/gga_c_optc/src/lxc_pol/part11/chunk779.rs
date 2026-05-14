//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 779/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk779<F: Float>(t15855: F, t3883: F, t1514: F, t3183: F, t2667: F, t3117: F, t5328: F) -> (F, F, F, F) {
    let t15856 = t15855 * t3883;
    let t15859 = t3183 * t1514;
    let t15860 = t15859 * t2667;
    let t15865 = t3117 * t5328;
    (t15856, t15859, t15860, t15865)
}
