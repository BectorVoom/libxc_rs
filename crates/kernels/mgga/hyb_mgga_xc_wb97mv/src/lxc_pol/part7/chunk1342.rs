//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1342/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1342<F: Float>(t12049: F, t7833: F, t1815: F, t515: F, t1567: F, t4097: F, t1128: F, t5722: F, t1157: F, t1298: F, t9872: F, t1114: F, t3678: F, t3677: F, t1111: F, t11896: F, t1291: F, t535: F) -> (F, F, F, F, F, F, F, F) {
    let t32784 = t7833 * t12049;
    let t32787 = t515 * t1815;
    let t32788 = t1567 * t4097;
    let t32790 = t5722 * t1128;
    let t32796 = t1157 * t9872 * t1298;
    let t32797 = t3678 * t1114;
    let t32798 = t3677 * t32797;
    let t32803 = t3678 * t1111;
    let t32804 = t3677 * t32803;
    let t32808 = t535 * t11896 * t1291;
    (t32784, t32787, t32788, t32790, t32796, t32798, t32804, t32808)
}
