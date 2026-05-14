//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1343/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1343<F: Float>(t11896: F, t1298: F, t535: F, t4533: F, t7899: F, t9831: F, t4545: F, t9851: F, t4619: F, t10172: F, t10177: F, t10186: F, t10190: F, t10194: F, t28388: F, t28844: F, t2887: F, t32784: F, t32787: F, t32788: F, t32790: F, t32796: F, t32798: F, t32804: F, t32808: F, t4588: F, t7832: F, t9846: F, t9862: F, sigma0: F) -> (F, F, F) {
    let t32812 = t535 * t11896 * t1298;
    let t32815 = t4533 * t7899;
    let t32818 = t4533 * t9831;
    let t32823 = t4545 * sigma0;
    let t32824 = t9851 * t32823;
    let t32827 = t4619 * t7899;
    let t32832 = 0.64e-1 * t7832 * t32784 + 0.37037037037037037037e0 * t32787 * t32788 * t32790 + 12.0 * t2887 * t4588 - 0.64e1 * t32796 * t32798 + 0.18432e-1 * t28388 * t9846 + 0.64e1 * t32796 * t32804 - 0.21504e-1 * t32808 * t9862 - 0.74666666666666666666e1 * t32812 * t32798 + 0.5632e-5 * t10177 * t32815 - 0.16896e-4 * t10186 * t32818 + 0.16896e-4 * t10190 * t32815 + 0.14222222222222222222e-2 * t10194 * t32824 + 0.24192e-1 * t28844 * t32827 - 0.5632e-5 * t10172 * t32818;
    (t32824, t32827, t32832)
}
