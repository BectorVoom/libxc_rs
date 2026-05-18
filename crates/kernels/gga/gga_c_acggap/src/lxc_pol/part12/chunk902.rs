//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 902/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk902<F: Float>(t1195: F, t7605: F, t1160: F, t7560: F, t3198: F, t1170: F, t30153: F) -> (F, F, F, F) {
    let t30675 = t7605 * t1195;
    let t30689 = t1160 * t7560;
    let t30690 = t30689 * t3198;
    let t30692 = t1170 * t30153;
    (t30675, t30689, t30690, t30692)
}
