//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1049/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1049<F: Float>(t18332: F, t18399: F, t18450: F, t18453: F, t18456: F, t18458: F, t18461: F, t18465: F, t18676: F, t18684: F, t18714: F, t2013: F, t2025: F, t2634: F, t5511: F, t7575: F, t7586: F, t782: F) -> (F,) {
    let t18717 = t18332 + t18399 + t18450 - t18453 - t18456 + 0.79959060960788076505e-2 * t18458 + 0.11993859144118211476e-1 * t2013 * t18461 + 0.27985671336275826777e-1 * t2013 * t18465 - 0.5397236614853195164e-1 * t7575 * t2025 - 0.2698618307426597582e-1 * t782 * t18676 + 0.14392630972941853771e0 * t7586 * t2025 - 0.17990788716177317213e-1 * t782 * t18684 - 0.14392630972941853771e0 * t2634 * t5511 + t18714;
    (t18717,)
}
