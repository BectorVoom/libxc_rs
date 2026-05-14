//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1227/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1227<F: Float>(t22709: F, t5108: F, t5110: F, t2132: F, t5135: F, t5100: F, t5174: F, t10878: F, t545: F, t6167: F, t6395: F, t6399: F, t6165: F, t6166: F, t6398: F, t2134: F, t2252: F) -> (F, F, F, F, F, F, F, F) {
    let t22711 = t5108 * t22709 * t5110;
    let t22721 = t5135 * t2132;
    let t22729 = t5100 * t5174;
    let t22731 = t545 * t10878;
    let t22732 = t22731 * t6167;
    let t22734 = t6395 * t6399;
    let t22737 = t6165 * t6398 * t6166;
    let t22739 = t2134 * t2252;
    (t22711, t22721, t22729, t22731, t22732, t22734, t22737, t22739)
}
