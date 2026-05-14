//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 531/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk531<F: Float>(t1032: F, t1113: F, t1092: F, t997: F, t1098: F, t1108: F, t360: F, t879: F, t368: F, t398: F, t384: F, t372: F, t1095: F, t3668: F, t395: F, t151: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3745 = t1032 * t1113;
    let t3747 = t997 * t1092;
    let t3750 = t997 * t1098;
    let t3752 = t1032 * t1108;
    let t3754 = t879 * t360;
    let t3756 = t398 * t368 * t3754;
    let t3757 = t384 * t3756;
    let t3759 = t879 * t372;
    let t3761 = t398 * t1095 * t3759;
    let t3762 = t384 * t3761;
    let t3764 = t395 * t3668;
    let t3765 = t151 * t3764;
    (t3745, t3747, t3750, t3752, t3754, t3756, t3757, t3759, t3761, t3762, t3765)
}
