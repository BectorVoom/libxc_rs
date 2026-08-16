//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 880/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk880<F: Float>(t1122: F, t8446: F, t3120: F, t3116: F, t3117: F, t3126: F, t2860: F, t3119: F, t3118: F, t22: F, t3145: F, t2850: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8447 = t8446 * t1122;
    let t8448 = t8447 * t3120;
    let t8449 = t3116 * t8448;
    let t8451 = t3117 * t3126;
    let t8452 = t8451 * t3120;
    let t8455 = t3119 * t2860;
    let t8456 = t3118 * t8455;
    let t8459 = t22 * t3145;
    let t8460 = t8459 * t1122;
    let t8461 = t3119 * t2850;
    (t8447, t8449, t8451, t8452, t8455, t8456, t8459, t8460, t8461)
}
