//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1125/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1125<F: Float>(t546: F, t6381: F, t19: F, t3003: F, t640: F, t668: F, t1975: F, t2004: F, t6395: F, t6388: F, t10: F, t21397: F, t1859: F, t28: F, t3004: F, t554: F, t559: F) -> (F, F, F, F, F, F, F, F, F) {
    let t21747 = t546 * t6381;
    let t21750 = t19 * t3003 * t640;
    let t21753 = t19 * t3003 * t668;
    let t21755 = t1975 * t2004;
    let t21759 = t546 * t6395;
    let t21761 = t546 * t6388;
    let t21775 = t21397 * t10;
    let t21778 = 1.0 / t28 / t1859;
    let t21793 = t554 * t3004 * t559;
    (t21747, t21750, t21753, t21755, t21759, t21761, t21775, t21778, t21793)
}
