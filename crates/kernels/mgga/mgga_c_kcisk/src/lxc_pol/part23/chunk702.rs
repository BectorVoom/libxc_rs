//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 702/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk702<F: Float>(t3484: F, t6229: F, t3482: F, t1440: F, t2059: F, t3797: F) -> (F, F, F) {
    let t6230 = t3484 * t6229;
    let t6231 = t3482 * t6230;
    let t6233 = t2059 * t1440;
    let t6234 = t3797 * t6233;
    (t6230, t6231, t6234)
}
