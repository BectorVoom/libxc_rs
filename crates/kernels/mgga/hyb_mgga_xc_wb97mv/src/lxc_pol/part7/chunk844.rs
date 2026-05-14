//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 844/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk844<F: Float>(t2261: F, t815: F, t2300: F, t834: F, t2198: F, t782: F, t2317: F, t2325: F, t2321: F, t837: F) -> (F, F, F, F, F) {
    let t6923 = t815 * t2261;
    let t6929 = t834 * t2300;
    let t6937 = t782 * t2198;
    let t6960 = t2317 * t2325;
    let t6965 = 1.0 / t2321 / t837;
    (t6923, t6929, t6937, t6960, t6965)
}
