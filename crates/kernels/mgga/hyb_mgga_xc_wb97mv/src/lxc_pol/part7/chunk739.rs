//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 739/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk739<F: Float>(t43: F, t1857: F, t1860: F, t3864: F, t1868: F, t574: F, t3854: F, t577: F, t1850: F, t3023: F, t571: F) -> (F, F, F, F, F) {
    let t45 = 0.135e1 < t43;
    let t3882 = t1857 * t1860 * t3864;
    let t3886 = t574 * t1868 * t3864;
    let t3890 = t574 * t577 * t3854;
    let t3893 = t1850 + t3023 / 81.0 - t571 * t3882 / 81.0 + t571 * t3886 / 27.0 - t571 * t3890 / 54.0;
    let t3894 = piecewise3(t45, t3893, 0.0);
    (t3882, t3886, t3890, t3893, t3894)
}
