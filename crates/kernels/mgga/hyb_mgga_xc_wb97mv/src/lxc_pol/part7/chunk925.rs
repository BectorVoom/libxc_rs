//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 925/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk925<F: Float>(t3155: F, t3157: F, t3159: F, t6717: F, t6723: F, t6727: F, t6741: F, t6743: F, t6747: F, t8519: F, t8523: F, t8528: F, t8530: F, t8531: F, t8536: F, t8538: F, t8546: F, t8548: F) -> (F,) {
    let t8557 = -t3155 * t8519 * t3159 / 24.0 - t3155 * t3157 * t8523 / 48.0 - 7.0 / 144.0 * t8528 * t8530 * t8531 + t3155 * t8536 * t8538 / 12.0 - t8546 + t8548 * t3157 * t8531 / 16.0 - t6741 / 64.0 + t6743 / 48.0 - t6747 - t6723 / 192.0 - t6727 / 144.0 + t6717 / 144.0;
    (t8557,)
}
