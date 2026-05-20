//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2954/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2954<F: Float>(t11994: F, t15769: F, t3151: F, t4772: F, t3298: F, t4746: F, t4891: F, t11744: F, t4834: F, t12012: F, t15822: F, t12009: F, t15823: F) -> (F, F, F, F, F, F) {
    let t53790 = t11994 * t15769;
    let t53792 = t4772 * t3151;
    let t53800 = t4746 * t3298 * t4891;
    let t53805 = t4834 * t11744;
    let t53807 = t15822 * t12012;
    let t53810 = t15823 * t12009;
    (t53790, t53792, t53800, t53805, t53807, t53810)
}
