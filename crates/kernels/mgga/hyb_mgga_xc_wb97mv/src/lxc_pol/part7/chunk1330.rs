//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1330/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1330<F: Float>(t259: F, t32476: F, t32500: F, t1046: F, t11614: F, t1041: F, t11612: F, t23669: F, t23674: F, t23675: F, t23679: F, t23680: F, t23684: F, t23959: F, t23964: F, t23970: F, t23975: F, t27805: F, t27807: F, t27818: F, t458: F, t491: F) -> (F, F) {
    let t32502 = (t32476 + t32500) * t259;
    let t32505 = t1046 * t11614;
    let t32507 = t1041 * t11612;
    let t32509 = t1046 * t11612;
    let t32511 = t1041 * t11614;
    let t32515 = -8.0 * t23669 + 120.0 * t27805 + 80.0 * t27807 - t23674 - 0.43374325201206959368e-1 * t23675 + t23679 + 0.96319466275353142156e0 * t23680 + t23684 + t458 * t32502 * t491 + t23959 - 8.0 * t32505 + 8.0 * t32507 - 8.0 * t32509 + 8.0 * t32511 - 480.0 * t23964 - t23970 + t23975 + 8.0 * t27818;
    (t32502, t32515)
}
