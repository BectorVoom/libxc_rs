//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1017/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1017<F: Float>(t1537: F, t7899: F, t1142: F, t2831: F, t535: F, t2893: F, t536: F, t1157: F) -> (F, F, F, F, F) {
    let t10178 = t1537 * t7899;
    let t10181 = t2831 * t1142;
    let t10182 = t535 * t10181;
    let t10185 = t536 * t2893;
    let t10186 = t1157 * t10185;
    (t10178, t10181, t10182, t10185, t10186)
}
