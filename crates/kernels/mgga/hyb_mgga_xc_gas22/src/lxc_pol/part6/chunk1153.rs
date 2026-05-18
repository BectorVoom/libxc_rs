//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1153/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1153<F: Float>(t1143: F, t3687: F, t524: F, t9573: F, t1166: F, t3660: F, t536: F, t9531: F, t2867: F, t525: F, t17: F, t7692: F) -> (F, F, F, F, F, F) {
    let t14765 = t1143 * t3687;
    let t14770 = t524 * t9573;
    let t14775 = t1166 * t3660;
    let t14815 = t536 * t9531;
    let t14818 = t2867 * t525;
    let t15041 = t7692 * t17;
    (t14765, t14770, t14775, t14815, t14818, t15041)
}
