//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1004/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1004<F: Float>(t2922: F, t3732: F, t3736: F, t3742: F, t1291: F, t2849: F, t3746: F, t2839: F, t2869: F, t509: F, t1122: F, tau0: F) -> (F, F, F, F, F, F, F) {
    let t10004 = t3732 * t2922;
    let t10007 = t3736 * t2922;
    let t10010 = t3742 * t2922;
    let t10013 = t1291 * t2849;
    let t10014 = t3746 * t10013;
    let t10021 = t1291 * t2839;
    let t10022 = t3746 * t10021;
    let t10029 = t509 * t2869;
    let t10034 = t1122 * tau0;
    (t10004, t10007, t10010, t10014, t10022, t10029, t10034)
}
