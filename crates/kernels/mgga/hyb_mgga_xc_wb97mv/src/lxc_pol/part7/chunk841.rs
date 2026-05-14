//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 841/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk841<F: Float>(t6759: F, t6814: F, t2282: F, t818: F, t262: F) -> (F, F, F, F) {
    let t6884 = 0.16068111111111111111e1 * t6759;
    let t6891 = 0.46308888888888888888e0 * t6814;
    let t6902 = 1.0 / t2282 / t818;
    let t6903 = t262 * t6902;
    (t6884, t6891, t6902, t6903)
}
