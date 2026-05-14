//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1122/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1122<F: Float>(t26796: F, t303: F, t6614: F, t1092: F, t27788: F, t95664: F, t15573: F, t29151: F, t7788: F, t18502: F, t7726: F, t1749: F, t5013: F, t1014: F, t28966: F, t19811: F, t7718: F, t9370: F) -> (F, F, F, F, F, F, F) {
    let t100619 = t303 * t26796 * t6614;
    let t100622 = t1092 * t95664 * t27788;
    let t100629 = t7788 * t15573 * t29151;
    let t100636 = t303 * t7726 * t18502;
    let t100641 = t303 * t1749 * t5013;
    let t100643 = t1014 * t28966;
    let t100646 = t9370 * t7718 * t19811;
    (t100619, t100622, t100629, t100636, t100641, t100643, t100646)
}
