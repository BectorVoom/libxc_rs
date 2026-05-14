//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1129/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1129<F: Float>(t32652: F, t9382: F, t9368: F, t3417: F, t397: F, t9380: F, t9379: F, t15705: F, t9364: F, t32633: F, t3368: F) -> (F, F, F, F, F, F, F) {
    let t32653 = t32652 * t9382;
    let t32655 = t32652 * t9368;
    let t32658 = t397 * t9380 * t3417;
    let t32659 = t9379 * t32658;
    let t32661 = t15705 * t9364;
    let t32662 = t32661 * t32633;
    let t32664 = t3368 * t9364;
    (t32653, t32655, t32658, t32659, t32661, t32662, t32664)
}
