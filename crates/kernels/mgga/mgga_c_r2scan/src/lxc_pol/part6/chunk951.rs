//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 951/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk951<F: Float>(t5021: F, t5872: F, t5874: F, t5871: F, t5878: F) -> (F, F) {
    let t7025 = 4.0 * t5021;
    let t7026 = 1584.0 * t5872;
    let t7027 = 1872.0 * t5874;
    let t7028 = t5871 - t7026 - t7027 + t5878;
    (t7025, t7028)
}
