//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1126/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1126<F: Float>(t1768: F, t303: F, t96210: F, t19680: F, t4554: F, t95911: F, t19571: F, t3200: F, t92808: F, t19723: F, t96249: F, t29172: F, t7784: F, t1134: F, t6482: F, t100284: F, t100314: F, t100389: F, t11020: F, t20684: F, t2197: F, t26960: F, t28113: F, t28118: F, t28123: F, t28125: F, t7779: F, t96917: F, t96926: F) -> (F, F, F, F, F, F) {
    let t100704 = t303 * t96210 * t1768;
    let t100707 = t4554 * t95911 * t19680;
    let t100736 = t3200 * t92808 * t19571;
    let t100741 = t3200 * t96249 * t19723;
    let t100746 = t29172 * t7784;
    let t100749 = t303 * t6482 * t1134;
    let t100751 = 0.46336805555555555556e-3 * t96917 * t28118 + 0.30918233506944444445e-4 * t96926 * t28113 - 0.30891203703703703704e-3 * t96917 * t28125 + 0.46336805555555555556e-3 * t26960 * t100284 + 0.15445601851851851852e-3 * t26960 * t11020 * t28123 * t100314 - 0.15476481481481481481e-2 * t100736 + 0.15445601851851851852e-3 * t26960 * t100389 - 0.51588271604938271603e-3 * t100741 + 0.92673611111111111112e-3 * t20684 * t7779 * t2197 - 0.11584201388888888889e-3 * t100746 + 0.11607361111111111111e-2 * t100749;
    (t100704, t100707, t100736, t100741, t100749, t100751)
}
