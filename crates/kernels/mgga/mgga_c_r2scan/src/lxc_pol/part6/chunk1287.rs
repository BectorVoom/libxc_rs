//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1287/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1287<F: Float>(t6127: F, t7604: F, t2133: F, t22709: F, t8093: F, t1543: F, t2526: F, t2620: F, t6182: F, t481: F, t7088: F, t1569: F, t24118: F, t2155: F, t22962: F, t1553: F, t938: F) -> (F, F, F, F, F, F, F, F) {
    let t24136 = t7604 * t6127;
    let t24141 = t2133 * t22709 * t8093;
    let t24145 = t2526 * t1543;
    let t24150 = t6182 * t2620;
    let t24156 = t7088 * t481;
    let t24161 = t24118 * t1569;
    let t24163 = t2155 * t22962 * t24161;
    let t24165 = t938 * t1553;
    (t24136, t24141, t24145, t24150, t24156, t24161, t24163, t24165)
}
