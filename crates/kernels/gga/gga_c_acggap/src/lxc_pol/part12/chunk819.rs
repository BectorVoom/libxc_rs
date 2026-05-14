//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 819/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk819<F: Float>(t30228: F, t601: F, t30174: F, t151: F, t56: F, t593: F, t606: F, t30225: F, t425: F, t1195: F, t7605: F, t1160: F, t7560: F, t3198: F, t1170: F, t30153: F) -> (F, F, F, F, F, F, F, F) {
    let t30663 = t30228 * t601;
    let t30665 = 1.0 / t30174;
    let t30668 = t151 * t593 * t30665 * t56;
    let t30669 = t30668 * t601;
    let t30671 = t30668 * t606;
    let t30673 = t30225 * t425;
    let t30675 = t7605 * t1195;
    let t30689 = t1160 * t7560;
    let t30690 = t30689 * t3198;
    let t30692 = t1170 * t30153;
    (t30663, t30669, t30671, t30673, t30675, t30689, t30690, t30692)
}
