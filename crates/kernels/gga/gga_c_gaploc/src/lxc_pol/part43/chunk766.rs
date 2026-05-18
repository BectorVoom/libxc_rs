//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 766/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk766<F: Float>(t1397: F, t9301: F, t30208: F, t493: F, t1339: F, t29969: F, t1406: F, t6575: F, t29984: F, t1381: F, t3141: F, t2754: F, t874: F) -> (F, F, F, F, F, F, F) {
    let t31182 = t1397 * t9301;
    let t31300 = t493 * t30208;
    let t31308 = t1339 * t29969;
    let t31356 = t1406 * t6575;
    let t31399 = t1339 * t29984;
    let t31428 = t3141 * t1381;
    let t31585 = t2754 * t874;
    (t31182, t31300, t31308, t31356, t31399, t31428, t31585)
}
