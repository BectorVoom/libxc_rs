//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1251/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1251<F: Float>(t1386: F, t16831: F, t4121: F, t491: F, t556: F, t3245: F, t8168: F, t12286: F, t6140: F, t1598: F, t51799: F, t1014: F, t28412: F) -> (F, F, F, F, F, F) {
    let t98653 = t16831 * t1386;
    let t98661 = t4121 * t491 * t556;
    let t98719 = t3245 * t8168;
    let t98721 = t12286 * t6140;
    let t98733 = t51799 * t1598;
    let t98743 = t1014 * t28412;
    (t98653, t98661, t98719, t98721, t98733, t98743)
}
