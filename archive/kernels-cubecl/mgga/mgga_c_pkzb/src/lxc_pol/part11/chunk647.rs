//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 647/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk647<F: Float>(t1137: F, t3645: F, t2105: F, t3521: F, t3523: F, t3527: F, t3553: F, t3556: F, t3612: F, t3614: F, t3616: F, t3620: F, t3624: F, t3628: F) -> (F, F, F) {
    let t3646 = t1137 * t3645;
    let t3647 = t2105 * t3646;
    let t3650 = -t3521 + t3523 - t3527 + t3553 + t3556 + t3612 + t3614 - t3616 + t3620 - t3624 - t3628;
    (t3646, t3647, t3650)
}
