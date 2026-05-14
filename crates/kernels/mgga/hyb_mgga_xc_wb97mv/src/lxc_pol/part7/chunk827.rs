//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 827/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk827<F: Float>(t140: F, t35: F, t6129: F, t6134: F, t703: F, t1852: F, t2068: F, t2073: F, t2077: F, t696: F, t700: F, t17: F, t2064: F, t699: F) -> (F, F, F, F, F, F, F, F) {
    let t6515 = 14.0 / 243.0 * t35 * t6129 * t140;
    let t6516 = t6134 * t703;
    let t6518 = t1852 * t2068;
    let t6520 = t1852 * t2073;
    let t6522 = t1852 * t2077;
    let t6525 = 1.0 / t696 / t700;
    let t6526 = t17 * t6525;
    let t6528 = 1.0 / t2064 / t699;
    (t6515, t6516, t6518, t6520, t6522, t6525, t6526, t6528)
}
