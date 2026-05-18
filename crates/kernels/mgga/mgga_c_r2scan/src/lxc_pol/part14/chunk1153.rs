//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1153/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1153<F: Float>(t40061: F, t546: F, t25968: F, t39841: F, t565: F, t25962: F, t10728: F, t7258: F, t1592: F, t24786: F, t3308: F, t39960: F) -> (F, F, F, F, F) {
    let t40062 = t546 * t40061;
    let t40064 = t40062 * t39841 * t25968;
    let t40066 = t565 * t40061;
    let t40068 = t40066 * t39841 * t25962;
    let t40070 = t10728 * t7258;
    let t40073 = t1592 * t3308 * t24786;
    let t40075 = t546 * t39960;
    (t40064, t40068, t40070, t40073, t40075)
}
