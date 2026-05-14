//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1044/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1044<F: Float>(t10699: F, t10703: F, t10708: F, t10712: F, t10715: F, t10719: F, t10723: F, t10726: F, t10730: F, t10735: F, t10739: F, t3040: F, t571: F, t6515: F, t6516: F, t8637: F, t8640: F, t8642: F, t8643: F) -> (F,) {
    let t10742 = -t6515 - 2.0 / 243.0 * t6516 - 4.0 / 243.0 * t8637 + t8640 - t8642 - 2.0 / 81.0 * t8643 + t10699 / 243.0 - 5.0 / 243.0 * t571 * t10703 + 2.0 / 27.0 * t571 * t10708 + 4.0 / 81.0 * t3040 * t10712 - t10715 / 81.0 - t571 * t10719 / 9.0 - 4.0 / 27.0 * t3040 * t10723 + t10726 / 162.0 - t571 * t10730 / 81.0 + t571 * t10735 / 27.0 - t571 * t10739 / 54.0;
    (t10742,)
}
