//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 586/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk586<F: Float>(t2776: F, t458: F, t10: F, t1056: F, t1096: F, t16: F, t1846: F, t488: F) -> (F, F, F, F) {
    let t2777 = t458 * t2776;
    let t2778 = t1056 * t10;
    let t2779 = t2778 * t1096;
    let t2782 = t16 * t1846 * t488;
    (t2777, t2778, t2779, t2782)
}
