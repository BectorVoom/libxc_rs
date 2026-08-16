//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 702/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk702<F: Float>(t1111: F, t604: F, t1181: F, t7426: F, t2070: F, t7433: F, t2450: F, t7336: F) -> (F, F, F, F, F) {
    let t7569 = t604 * t1111;
    let t7570 = t1181 * t7569;
    let t7571 = t7426 * t7570;
    let t7573 = t7433 * t2070;
    let t7575 = t2450 * t7336;
    (t7569, t7570, t7571, t7573, t7575)
}
