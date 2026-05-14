//! HYB_MGGA_XC_WB97MV lxc pol — lxc_pol part 7 (v4rho4_2) CSE chunk 1346/1375 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_wb97mv_lxc_pol_part7_v4rho4_2_chunk1346<F: Float>(t1801: F, t24434: F, t297: F, t396: F, t4083: F, t1522: F, t10116: F, t27911: F, t3732: F, t1157: F, t32679: F, t2822: F, t4571: F, t10080: F, t9767: F, t10079: F, t1112: F, t11879: F, t1528: F, t24245: F, t27899: F, t2825: F, t2849: F, t32760: F, t32778: F, t32870: F, t32875: F, t3685: F, t3771: F, t3785: F, t4529: F, t511: F, t532: F, t7838: F, t7848: F, t7897: F, t7903: F, t9784: F) -> (F, F, F) {
    let t32893 = t24434 * t396 * t4083 * t297 * t1801;
    let t32896 = t1522 * t297;
    let t32897 = t32896 * t1801;
    let t32898 = t10116 * t32897;
    let t32901 = t3732 * t27911;
    let t32915 = t1157 * t32679;
    let t32918 = t4571 * t2822;
    let t32926 = t10080 * t9767;
    let t32929 = 0.288e-3 * t7838 * t32870 - 0.33792e-7 * t3685 * t32875 + 0.144e-2 * t7848 * t3732 * t27899 + 0.294912e-9 * t3685 * t32893 - 0.27648e-4 * t7903 * t32898 - 0.192e-3 * t7897 * t32901 - 0.3072e-5 * t7897 * t32898 + 0.294912e-9 * t3771 * t32893 + 120.0 * t511 * t532 * t4529 * t2849 + 400.0 / 9.0 * t1112 * t1528 * t3785 + 0.9216e-2 * t32915 * t32778 - 0.192e-3 * t32918 * t2825 + 0.1024e-2 * t24245 * t32760 - 0.48e0 * t11879 * t10080 * t9784 + 0.576e0 * t10079 * t32926;
    (t32901, t32926, t32929)
}
