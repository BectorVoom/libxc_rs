//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1314/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1314<F: Float>(t11332: F, t986: F, t11383: F, t23392: F, t2590: F, t31668: F, t31671: F, t31673: F, t31676: F, t31679: F, t31682: F, t31685: F, t31688: F, t31691: F, t31693: F, t31697: F, t31700: F, t31704: F, t31706: F, t31709: F, t32035: F, t4376: F, t987: F, t995: F, t996: F) -> (F,) {
    let t32080 = t11332 * t986;
    let t32085 = t31668 + t31671 + t31673 + t31676 + t31679 + t31682 + t31685 + t31688 + t31691 - t31693 - t31697 - t31700 - t31704 - t31706 - t31709 + 0.5848223622634646207e0 * t987 * t32035 * t995 + 0.17315859105681463759e2 * t23392 * t4376 + 0.11696447245269292414e1 * t32080 * t996 + 0.5848223622634646207e0 * t11383 * t2590;
    (t32085,)
}
