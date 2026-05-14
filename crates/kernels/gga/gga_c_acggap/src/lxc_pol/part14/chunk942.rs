//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 942/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk942<F: Float>(t5873: F, t7822: F, t7493: F, t8480: F, t8648: F, t6332: F, t8511: F, t1755: F, t30540: F, t7433: F, t9674: F, t1801: F, t7329: F, t1165: F, t5567: F, t7426: F, t8600: F) -> (F, F, F, F, F, F, F) {
    let t38701 = t7822 * t5873;
    let t38704 = t7493 * t8480 * t8648;
    let t38706 = t8511 * t6332;
    let t38709 = t30540 * t1755;
    let t38711 = t7433 * t9674;
    let t38713 = t7329 * t1801;
    let t38717 = t7426 * t1165 * t8600 * t5567;
    (t38701, t38704, t38706, t38709, t38711, t38713, t38717)
}
