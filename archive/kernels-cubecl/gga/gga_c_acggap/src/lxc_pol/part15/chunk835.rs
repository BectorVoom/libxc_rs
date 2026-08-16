//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 835/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk835<F: Float>(t1181: F, t9719: F, t7575: F, t1089: F, t1459: F, t9563: F, t598: F, t142: F, t1866: F, t7436: F, t7815: F, t2030: F) -> (F, F, F, F, F, F, F, F) {
    let t9720 = t1181 * t9719;
    let t9721 = t7575 * t9720;
    let t9724 = t1089 * t1459 * t9563;
    let t9725 = t598 * t9724;
    let t9727 = t142 * t1866;
    let t9728 = t7436 * t9727;
    let t9730 = t7815 * t1866;
    let t9731 = t2030 * t9730;
    (t9720, t9721, t9724, t9725, t9727, t9728, t9730, t9731)
}
