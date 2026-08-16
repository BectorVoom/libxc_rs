//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 822/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk822<F: Float>(t12630: F, t196: F, t852: F, t989: F, t816: F, t179: F, t2925: F, t15: F, t197: F, t2861: F, t183: F, t3: F) -> (F, F, F, F, F) {
    let t12631 = t12630 * t196;
    let t12636 = t852 * t989;
    let t12637 = t12636 * t816;
    let t12640 = t2925 * t179;
    let t12641 = t12640 * t15;
    let t12644 = t197 * t2861;
    let t12645 = t183 * t3;
    (t12631, t12637, t12641, t12644, t12645)
}
