//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1089/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1089<F: Float>(t3275: F, t3276: F, t40691: F, t38282: F, t38298: F, t38303: F, t38308: F, t38312: F, t40659: F, t40662: F, t40666: F, t40670: F, t40672: F, t40679: F, t40683: F, t40686: F, t40690: F) -> (F, F) {
    let t40694 = 5.0 / 16.0 * t3275 * t3276 * t40691;
    let t40695 = -t38282 + 0.34200192530023447503e-6 * t40659 + t40662 - t40666 + t40670 - 0.35220688045884876043e-2 * t40672 - t38298 - 0.14408463291498358381e-2 * t38303 + 0.36021158228745895953e-3 * t38308 + t38312 - t40679 - t40683 - t40686 - t40690 + t40694;
    (t40694, t40695)
}
