//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 864/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk864<F: Float>(t16220: F, t7: F, t171: F, t22: F, t5815: F, t12760: F, t139: F, t41: F, t11: F, t172: F, t397: F, t963: F, t3483: F, t3776: F, t1341: F, t14264: F) -> (F, F, F, F, F, F, F, F) {
    let t16221 = t7 * t16220;
    let t16222 = t171 * t16221;
    let t16391 = t22 * t5815;
    let t16940 = t139 * t12760 * t41;
    let t18053 = t139 * t11 * t41;
    let t18080 = t172 * t41;
    let t18081 = t139 * t18080;
    let t18681 = t397 * t963;
    let t18984 = t3483 * t3776;
    let t18997 = t14264 * t1341;
    (t16222, t16391, t16940, t18053, t18081, t18681, t18984, t18997)
}
