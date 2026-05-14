//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 767/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk767<F: Float>(t2297: F, t513: F, t4262: F, t2030: F, t1755: F, t7822: F, t1761: F, t1859: F, t604: F, t1181: F, t7575: F, t1089: F, t1459: F, t9563: F, t598: F, t142: F, t1866: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t9711 = t2297 * t513;
    let t9712 = t4262 * t9711;
    let t9713 = t2030 * t9712;
    let t9715 = t7822 * t1755;
    let t9717 = t7822 * t1761;
    let t9719 = t604 * t1859;
    let t9720 = t1181 * t9719;
    let t9721 = t7575 * t9720;
    let t9724 = t1089 * t1459 * t9563;
    let t9725 = t598 * t9724;
    let t9727 = t142 * t1866;
    (t9711, t9712, t9713, t9715, t9717, t9719, t9720, t9721, t9724, t9725, t9727)
}
