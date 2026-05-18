//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1167/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1167<F: Float>(t7407: F, t92890: F, t2061: F, t22: F, t25402: F, t93140: F, t25310: F, t26506: F, t26485: F, t93364: F, t2829: F, t689: F, t7384: F) -> (F, F, F, F, F, F) {
    let t95543 = t92890 * t7407;
    let t95546 = t25402 * t2061 * t22;
    let t95548 = F::new(0.51727911450665971904e-3) * t93140 * t95546;
    let t95551 = t25310 * t26506;
    let t95553 = t93364 * t26485;
    let t95556 = t689 * t7384 * t2829;
    (t95543, t95546, t95548, t95551, t95553, t95556)
}
