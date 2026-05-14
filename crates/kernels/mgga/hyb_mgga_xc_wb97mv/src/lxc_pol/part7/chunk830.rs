//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 830/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk830<F: Float>(t6623: F, t180: F, t2135: F, t2120: F, t745: F, t172: F, t10: F, t2065: F, t214: F, t2162: F, t2003: F, t685: F) -> (F, F, F, F, F, F, F) {
    let t6624 = 1.0 / t6623;
    let t6628 = t180 * t2135;
    let t6648 = 1.0 / t2120 / t745;
    let t6659 = 1.0 / t6623 / t172;
    let t6695 = t2065 * t10;
    let t6701 = t2162 * t214;
    let t6715 = t2003 * t685;
    (t6624, t6628, t6648, t6659, t6695, t6701, t6715)
}
