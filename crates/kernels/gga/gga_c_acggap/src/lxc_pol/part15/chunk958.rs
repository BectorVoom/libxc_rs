//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 958/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk958<F: Float>(t6332: F, t8511: F, t1755: F, t30540: F, t7433: F, t9674: F, t1801: F, t7329: F, t1165: F, t5567: F, t7426: F, t8600: F, t5572: F, t7575: F, t30120: F, t9645: F) -> (F, F, F, F, F, F, F) {
    let t38706 = t8511 * t6332;
    let t38709 = t30540 * t1755;
    let t38711 = t7433 * t9674;
    let t38713 = t7329 * t1801;
    let t38717 = t7426 * t1165 * t8600 * t5567;
    let t38721 = t7575 * t1165 * t8600 * t5572;
    let t38723 = t30120 * t9645;
    (t38706, t38709, t38711, t38713, t38717, t38721, t38723)
}
