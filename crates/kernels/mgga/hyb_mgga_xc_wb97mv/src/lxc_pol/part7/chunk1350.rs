//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1350/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1350<F: Float>(t1291: F, t3677: F, t9896: F, t1537: F, t2849: F, t3736: F, t11861: F, t1795: F, t3686: F, t3732: F, t2839: F, t3746: F, t4077: F, t11821: F, t2922: F, t11826: F) -> (F, F, F, F, F, F, F, F) {
    let t33055 = t3677 * t9896 * t1291;
    let t33062 = t1537 * t2849;
    let t33063 = t3736 * t33062;
    let t33067 = t3686 * t11861 * t1795;
    let t33074 = t3732 * t33062;
    let t33077 = t1537 * t2839;
    let t33082 = t3746 * t4077 * t2849;
    let t33085 = t11821 * t2922;
    let t33088 = t11826 * t2922;
    (t33055, t33063, t33067, t33074, t33077, t33082, t33085, t33088)
}
