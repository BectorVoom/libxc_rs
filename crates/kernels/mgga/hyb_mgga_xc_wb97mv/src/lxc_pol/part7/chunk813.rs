//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 813/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk813<F: Float>(t4550: F, t498: F, t1537: F, t3799: F, t3774: F, t2894: F, t4558: F, t1519: F, t1522: F) -> (F, F, F, F, F) {
    let t4608 = t498 * t4550;
    let t4610 = t1537 * t3799;
    let t4613 = t1537 * t3774;
    let t4616 = t2894 * t4558;
    let t4619 = t1519 * t1522;
    (t4608, t4610, t4613, t4616, t4619)
}
