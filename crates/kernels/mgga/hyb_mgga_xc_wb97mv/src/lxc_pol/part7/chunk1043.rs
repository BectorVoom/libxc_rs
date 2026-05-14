//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1043/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1043<F: Float>(t3: F, t3188: F, t3189: F, t1852: F, t3985: F, t2065: F, t3979: F, t3194: F, t674: F, t3195: F, t3990: F, t3988: F, t2033: F, t10621: F, t698: F, t701: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t10712 = t3188 * t3189 * t3;
    let t10715 = t1852 * t3985;
    let t10717 = t2065 * t3979;
    let t10719 = t3194 * t10717 * t674;
    let t10723 = t3194 * t3195 * t3;
    let t10726 = t1852 * t3990;
    let t10728 = t2065 * t3988;
    let t10730 = t3188 * t10728 * t674;
    let t10733 = t2033 * t3988;
    let t10735 = t3194 * t10733 * t674;
    let t10739 = t698 * t701 * t10621;
    (t10712, t10715, t10717, t10719, t10723, t10726, t10728, t10730, t10733, t10735, t10739)
}
