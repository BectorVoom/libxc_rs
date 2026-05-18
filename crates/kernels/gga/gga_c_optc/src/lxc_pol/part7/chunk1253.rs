//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1253/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1253<F: Float>(t25836: F, t2769: F, t19: F, t24567: F, t1659: F, t2715: F, t8072: F, t24550: F, t953: F, t2812: F, t8044: F, t8143: F) -> (F, F, F, F, F, F) {
    let t25877 = t2769 * t25836;
    let t25878 = t24567 * t19;
    let t25883 = t1659 * t25836;
    let t25888 = t8072 * t2715;
    let t25902 = t953 * t24550;
    let t25905 = t2812 * t8143 * t8044;
    (t25877, t25878, t25883, t25888, t25902, t25905)
}
