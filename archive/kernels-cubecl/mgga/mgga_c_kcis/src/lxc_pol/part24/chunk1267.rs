//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1267/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1267<F: Float>(t1768: F, t303: F, t96210: F, t19680: F, t4554: F, t95911: F, t19571: F, t3200: F, t92808: F, t19723: F, t96249: F, t29172: F, t7784: F) -> (F, F, F, F, F) {
    let t100704 = t303 * t96210 * t1768;
    let t100707 = t4554 * t95911 * t19680;
    let t100736 = t3200 * t92808 * t19571;
    let t100741 = t3200 * t96249 * t19723;
    let t100746 = t29172 * t7784;
    (t100704, t100707, t100736, t100741, t100746)
}
