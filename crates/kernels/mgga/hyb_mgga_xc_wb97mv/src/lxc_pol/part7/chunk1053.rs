//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1053/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1053<F: Float>(t4115: F, t679: F, t4068: F, t549: F, t136: F, t8601: F, t8604: F, t8607: F, t8617: F, t8620: F, t8623: F, t8628: F, t8632: F, t8852: F, t8860: F, t8862: F, t8864: F, t8870: F) -> (F, F) {
    let t10892 = t4115 * t679;
    let t10894 = t549 * t4068;
    let t10895 = t136 * t10894;
    let t10903 = -t10892 / 64.0 - t10895 / 64.0 - t8601 - t8604 + t8607 / 48.0 + t8617 / 16.0 + t8620 / 48.0 - t8623 - t8628 - t8632 + t8852 / 144.0 - t8860 + t8862 / 48.0 + t8864 / 48.0 - t8870;
    (t10894, t10903)
}
