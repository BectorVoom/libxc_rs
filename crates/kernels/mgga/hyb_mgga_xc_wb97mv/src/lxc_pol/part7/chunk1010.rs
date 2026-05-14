//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1010/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1010<F: Float>(t2901: F, t3799: F, t2952: F, t9838: F, t3791: F, t7833: F, t3795: F, t2848: F, t529: F, t1291: F, t2893: F, t2895: F, t2869: F, t3746: F, t2873: F, t1519: F, tau0: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10088 = t3799 * t2901;
    let t10091 = t2952 * t9838;
    let t10092 = t7833 * t3791;
    let t10095 = t7833 * t3795;
    let t10098 = t2848 * tau0;
    let t10099 = t529 * t10098;
    let t10102 = t1291 * t2893;
    let t10103 = t10102 * t2895;
    let t10106 = t1291 * t2869;
    let t10107 = t3746 * t10106;
    let t10110 = t1291 * t2873;
    let t10111 = t3746 * t10110;
    let t10116 = t2893 * t1519;
    (t10088, t10091, t10092, t10095, t10098, t10099, t10103, t10107, t10111, t10116)
}
