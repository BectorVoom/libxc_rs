//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1025/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1025<F: Float>(t22837: F, t397: F, t5544: F, t32269: F, t5589: F, t92354: F, t37939: F, t409: F, t25754: F, t32274: F, t1728: F, t70: F, t5569: F, t5572: F, t22825: F, t22833: F) -> (F, F, F, F, F, F, F) {
    let t92616 = t5544 * t22837 * t397;
    let t92639 = t92354 * t5589 * t32269;
    let t92642 = t37939 * t409;
    let t92644 = t25754 * t32274;
    let t92652 = t1728 * t70;
    let t92654 = t5569 * t92652 * t5572;
    let t92666 = t22833 * t22825;
    (t92616, t92639, t92642, t92644, t92652, t92654, t92666)
}
