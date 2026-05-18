//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1172/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1172<F: Float>(t10719: F, t8198: F, t1575: F, t269: F, t546: F, t25968: F, t39841: F, t565: F, t25962: F, t10728: F, t7258: F, t1592: F, t24786: F, t3308: F) -> (F, F, F, F, F) {
    let t40059 = t8198 * t10719;
    let t40061 = t1575 * t269;
    let t40062 = t546 * t40061;
    let t40064 = t40062 * t39841 * t25968;
    let t40066 = t565 * t40061;
    let t40068 = t40066 * t39841 * t25962;
    let t40070 = t10728 * t7258;
    let t40073 = t1592 * t3308 * t24786;
    (t40059, t40064, t40068, t40070, t40073)
}
