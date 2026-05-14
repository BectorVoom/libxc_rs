//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 553/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk553<F: Float>(t1056: F, t1440: F, t3797: F, t3796: F, t3482: F, t139: F, t157: F, t79: F) -> (F, F, F, F, F) {
    let t3798 = t1056 * t1440;
    let t3799 = t3797 * t3798;
    let t3800 = t3796 * t3799;
    let t3801 = t3482 * t3800;
    let t3805 = t139 * t157 * t79;
    (t3798, t3799, t3800, t3801, t3805)
}
