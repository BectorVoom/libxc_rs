//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1114/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1114<F: Float>(t16642: F, t4160: F, t94425: F, t27455: F, t28369: F, t94398: F, t94402: F, t98242: F, t98378: F, t98381: F, t98383: F, t98387: F, t98388: F, t98390: F, t98392: F, t1467: F, t1928: F) -> (F, F, F) {
    let t98396 = t4160 * t94425 * t16642;
    let t98400 = -0.33163888888888888888e-2 * t98378 + t98381 - t98383 + 0.46336805555555555556e-3 * t28369 * t27455 - t98387 + 0.12356481481481481481e-2 * t98388 + 0.15445601851851851852e-3 * t98390 - 0.12378114784505208333e-4 * t98392 * t98242 - 0.22109259259259259258e-2 * t98396 - 0.7722800925925925926e-4 * t94398 - 0.10297067901234567901e-3 * t94402;
    let t98409 = t1467 * t1928;
    (t98396, t98400, t98409)
}
