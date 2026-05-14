//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 901/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk901<F: Float>(t2158: F, t6407: F, t2155: F, t5116: F, t1415: F, t511: F, t2162: F, t2164: F, t2078: F, t784: F) -> (F, F, F, F, F) {
    let t6408 = t6407 * t2158;
    let t6410 = t2155 * t5116;
    let t6412 = t1415 * t511;
    let t6415 = 0.89443204944342177673e-3 * t6412 * t2162 * t2164;
    let t6416 = t2078 * t784;
    (t6408, t6410, t6412, t6415, t6416)
}
