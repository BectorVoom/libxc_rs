//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1356/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1356<F: Float>(t2848: F, t4550: F, t1114: F, t11780: F, t11781: F, t14407: F, t32710: F, t1815: F, t522: F, t1616: F, t32788: F, t10080: F, t1148: F, t11680: F, t27965: F, t2839: F, t2849: F, t28585: F, t28648: F, t28749: F, t32711: F, t32878: F, t32901: F, t4594: F, t505: F, t7818: F, t7838: F, t7848: F, t7897: F, t7903: F, t7938: F, t9985: F) -> (F, F) {
    let t33284 = t2848 * t4550;
    let t33292 = t11780 * t11781 * t1114;
    let t33295 = t32710 * t14407;
    let t33300 = t522 * t1815;
    let t33302 = t33300 * t32788 * t1616;
    let t33322 = 120.0 * t7938 * t4594 * t2839 + 252.0 * t1148 * t33284 * t2849 - 6.0 * t505 * t11680 * t2849 - 0.27648e-4 * t7903 * t33292 - 0.36864e-4 * t27965 * t33295 + 0.36864e-4 * t28585 * t32711 + 0.14814814814814814814e1 * t7897 * t33302 + 0.11111111111111111111e1 * t7838 * t33302 + 0.55555555555555555556e1 * t7848 * t33302 + 0.13333333333333333333e2 * t7903 * t33302 - 0.12288e-4 * t28749 * t33295 + 0.77777777777777777778e1 * t7818 * t33302 + 0.576e1 * t28648 * t10080 * t9985 - 0.1728e-2 * t7903 * t32901 - 0.1728e-2 * t7903 * t32878;
    (t33292, t33322)
}
