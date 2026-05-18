//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 815/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk815<F: Float>(t4536: F, t4539: F, t1214: F, t5474: F, t1213: F, t5440: F, t490: F, t4310: F, t4314: F, t24: F, t5285: F, t1111: F) -> (F, F, F, F, F, F, F) {
    let t15181 = t4536 * t4539;
    let t15200 = t5474 * t1214;
    let t15204 = t5440 * t1213;
    let t15205 = t490 * t15204;
    let t15225 = t4310 * t4314;
    let t15227 = t24 * t5285;
    let t15228 = t1111 * t15227;
    (t15181, t15200, t15204, t15205, t15225, t15227, t15228)
}
