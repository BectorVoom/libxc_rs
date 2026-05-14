//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 776/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk776<F: Float>(t1147: F, t135: F, t2156: F, t273: F, t3521: F, t3523: F, t3527: F, t3553: F, t3556: F, t3612: F, t3614: F, t3616: F, t3620: F, t3624: F, t3628: F, t3698: F, t805: F) -> (F, F) {
    let t3702 = t1147 * t1147;
    let t3706 = -t135 * t2156 * t273 * t3702 + t135 * t273 * t3698 * t805 - t3521 + t3523 - t3527 + t3553 + t3556 + t3612 + t3614 - t3616 + t3620 - t3624 - t3628;
    (t3702, t3706)
}
