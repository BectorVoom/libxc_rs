//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1100/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1100<F: Float>(t1114: F, t4588: F, t2856: F, t4554: F, t1519: F, t3679: F, t1522: F, sigma2: F, tau0: F) -> (F, F, F, F, F, F, F) {
    let t11672 = t4588 * t1114;
    let t11680 = t2856 * t4554;
    let t11689 = t1519 * tau0;
    let t11690 = t11689 * t3679;
    let t11693 = t1522 * tau0;
    let t11694 = t11693 * t3679;
    let t11703 = t1519 * sigma2;
    (t11672, t11680, t11689, t11690, t11693, t11694, t11703)
}
