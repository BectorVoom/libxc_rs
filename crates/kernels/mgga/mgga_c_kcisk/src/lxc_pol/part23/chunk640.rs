//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 640/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk640<F: Float>(t3484: F, t5621: F, t3482: F, t1390: F, t470: F, t1056: F, t2059: F) -> (F, F, F, F) {
    let t5622 = t3484 * t5621;
    let t5623 = t3482 * t5622;
    let t5625 = t470 * t1390;
    let t5626 = t2059 * t1056;
    (t5622, t5623, t5625, t5626)
}
