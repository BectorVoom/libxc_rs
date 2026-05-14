//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1102/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1102<F: Float>(t15573: F, t29093: F, t7788: F, t1092: F, t28991: F, t92701: F, t18513: F, t2842: F, t7718: F, t29103: F, t3500: F, t19160: F, t26760: F, t19745: F, t19807: F, t1262: F, t30045: F, t5329: F, t6737: F) -> (F, F, F, F, F, F, F, F, F) {
    let t100129 = t15573 * t29093;
    let t100130 = t7788 * t100129;
    let t100133 = t1092 * t92701 * t28991;
    let t100136 = t2842 * t7718 * t18513;
    let t100139 = t7788 * t3500 * t29103;
    let t100142 = t2842 * t26760 * t19160;
    let t100145 = t2842 * t7718 * t19745;
    let t100148 = t2842 * t7718 * t19807;
    let t100152 = t5329 * t30045 * t6737 * t1262;
    (t100129, t100130, t100133, t100136, t100139, t100142, t100145, t100148, t100152)
}
